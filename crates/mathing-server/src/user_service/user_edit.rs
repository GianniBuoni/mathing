use sqlx::{PgPool, Postgres};

use crate::{get_duplicates::get_duplicates, prelude::mathing_proto::UserEdit};

use super::{user_get::user_get, user_row::UserPgRow, *};

impl MathingUserService {
    pub(super) async fn handle_edit(
        &self,
        req: Request<UserEditRequest>,
    ) -> Result<Response<UserEditResponse>, Status> {
        let req = req.into_inner();
        info!("{req:?}");

        let conn = DBconn::try_get().await?;
        let edit_reqs = Arc::<[UserEdit]>::from(req.user_edit);

        let users = tokio::time::timeout(DBconn::context(), user_edit(conn, edit_reqs))
            .await
            .map_err(|_| DbError::ContextError)??
            .into_iter()
            .collect();

        Ok(Response::new(UserEditResponse { users }))
    }
}

async fn user_edit(conn: &PgPool, reqs: Arc<[UserEdit]>) -> Result<Vec<UserPgRow>, DbError> {
    // validate old
    let old = Arc::<[String]>::from_iter(reqs.iter().cloned().map(|f| f.target));
    let _ = user_get(conn, old).await?;
    // validate new
    let new = Arc::<[String]>::from_iter(reqs.iter().cloned().map(|f| f.name));
    if let Some(found) = get_duplicates(new.clone()) {
        return Err(DbError::UniqueConstraint("users", found));
    }
    if let Ok(found) = user_get(conn, new.clone()).await {
        let found = found
            .iter()
            .map(|f| &f.name)
            .cloned()
            .collect::<Vec<String>>()
            .join(", ");
        return Err(DbError::UniqueConstraint("users", found.to_string()));
    }
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

    fn jon_to_blue() -> UserEdit {
        UserEdit {
            target: "jon".into(),
            name: "blue".into(),
        }
    }
    fn noodle_to_blue() -> UserEdit {
        UserEdit {
            target: "noodle".into(),
            name: "blue".into(),
        }
    }

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
    /// Test to ensure unique constraints of the table
    /// are upheld by the edit funcition.
    /// A client can successfully pass a name already in the database,
    /// which the server should respond with an error.
    async fn test_target_exists(conn: PgPool) -> anyhow::Result<()> {
        let want = DbError::UniqueConstraint("users", "blue".into());
        let reqs = vec![jon_to_blue(), noodle_to_blue()].into();
        let got = user_edit(&conn, reqs).await.map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }
    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    /// Test to ensure that old values are unique as well.
    /// Letting the server attempt to rename the same entries multiple times
    /// would result in undefined behavior.
    async fn test_repeat_target(conn: PgPool) -> anyhow::Result<()> {
        let want = DbError::UniqueConstraint("users", "jon".into());
        let reqs = vec![jon_to_blue(); 3].into();
        let got = user_edit(&conn, reqs).await.map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }
    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    /// Test to ensure that new values are unique as well.
    /// Letting the server attempt to rename multiple entries into the same name
    /// should return unique constrint error.
    async fn test_repeat_name(conn: PgPool) -> anyhow::Result<()> {
        let want = DbError::UniqueConstraint("users", "paul".into());
        let reqs = vec![
            UserEdit {
                target: "jon".into(),
                name: "paul".into(),
            },
            UserEdit {
                target: "noodle".into(),
                name: "paul".into(),
            },
        ]
        .into();
        let got = user_edit(&conn, reqs).await.map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }
    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    /// Test that expects a entry not found error.
    /// The database should check if the given name exists before
    /// attempting to edit it.
    async fn test_target_not_found(conn: PgPool) -> anyhow::Result<()> {
        let want = DbError::EntryNotFound("users", "paul".into());
        // These args will both be invalid, but
        // the non-existent user should be detected frist.
        let reqs = vec![UserEdit {
            target: "paul".into(),
            name: "blue".into(),
        }]
        .into();
        let got = user_edit(&conn, reqs).await.map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        };
        Ok(())
    }
}
