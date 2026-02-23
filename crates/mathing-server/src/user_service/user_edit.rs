use sqlx::{PgPool, Postgres};

use crate::prelude::mathing_proto::UserEdit;

use super::{user_row::UserPgRow, *};

impl MathingUserService {
    pub(super) async fn handle_edit(
        &self,
        req: Request<UserEditRequest>,
    ) -> Result<Response<UserEditResponse>, Status> {
        let req = req.into_inner();
        info!("{req:?}");

        let conn = DBconn::try_get().await?;
        let reqs = Arc::<[UserEdit]>::from(req.user_edit);

        let users = tokio::time::timeout(
            DBconn::context(),
            async || -> Result<Vec<UserPgRow>, Status> {
                validate_edit(conn, reqs.clone()).await?;
                Ok(user_edit(conn, reqs).await?)
            }(),
        )
        .await
        .map_err(|_| DbError::ContextError)??
        .into_iter()
        .collect();

        Ok(Response::new(UserEditResponse { users }))
    }
}

async fn validate_edit(conn: &PgPool, reqs: Arc<[UserEdit]>) -> Result<(), ClientError> {
    // validate old exists in database
    let old = Arc::<[String]>::from_iter(reqs.iter().cloned().map(|f| f.target));
    Validation::new(old, "users", "name")
        .args_exist()
        .validate(conn)
        .await?;
    // validate new does not exist in database
    let new = Arc::<[String]>::from_iter(reqs.iter().cloned().map(|f| f.name));
    Validation::new(new, "users", "name")
        .unique_constraint()
        .validate(conn)
        .await
}

async fn user_edit(conn: &PgPool, reqs: Arc<[UserEdit]>) -> Result<Vec<UserPgRow>, DbError> {
    // sql statement
    let mut q = sqlx::QueryBuilder::<Postgres>::new(
        "UPDATE users SET name = data.new_name, updated_at = CURRENT_TIMESTAMP FROM (",
    );
    q.push_values(reqs.iter().take(BIND_LIMIT / 2), |mut b, req| {
        b.push_bind(&req.target);
        b.push_bind(&req.name);
    });
    q.push(") AS data(target, new_name) WHERE users.name=data.target RETURNING *");
    // transaction
    let mut tx = conn.begin().await?;
    let q = q.build_query_as::<UserPgRow>();
    let rows = q.fetch_all(&mut *tx).await?;
    tx.commit().await?;

    Ok(rows)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::errors::expected_error;

    use super::*;

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    /// Basic test for editing a user name and if
    /// updated field is actually updated.
    async fn test_user_edit(conn: PgPool) -> anyhow::Result<()> {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let want = "paul";
        let (target, name) = ("jon".into(), want.into());
        let reqs = vec![UserEdit { target, name }].into();
        let got = user_edit(&conn, reqs).await?;
        let got = got.first().unwrap();

        assert_eq!(want, &got.name);
        assert_ne!(got.created_at, got.updated_at);
        Ok(())
    }
    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    /// Test to ensure that new values are unique to the database.
    /// Letting the server attempt to rename multiple entries into the same name
    /// should return unique constraint error.
    async fn test_name_exists(conn: PgPool) {
        let want = ClientError::UniqueConstraint("users".into(), "blue".into());
        let reqs = vec![UserEdit {
            target: "jon".into(),
            name: "blue".into(),
        }]
        .into();
        let got = validate_edit(&conn, reqs).await.map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    /// The database should check if the target name exists before
    /// attempting to edit it.
    async fn test_target_not_found(conn: PgPool) {
        let want = ClientError::EntryNotFound("users".into(), "paul".into());
        // These args will both be invalid, but
        // the non-existent user should be detected frist.
        let reqs = vec![UserEdit {
            target: "paul".into(),
            name: "blue".into(),
        }]
        .into();
        let got = validate_edit(&conn, reqs).await.map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        };
    }
}
