use sqlx::{PgPool, Postgres};

use super::{user_row::UserPgRow, *};

impl MathingUserService {
    pub(super) async fn handle_create(
        &self,
        req: Request<UserCreateRequest>,
    ) -> Result<Response<UserCreateResponse>, Status> {
        let req = req.into_inner();
        info!("{:?}", req);

        let conn = DBconn::try_get().await?;
        let names = Arc::<[String]>::from(req.names);

        let users = tokio::time::timeout(DBconn::context(), async {
            validate_create(conn, names.clone()).await?;
            Ok::<Vec<UserPgRow>, Status>(user_create(conn, names).await?)
        })
        .await
        .map_err(|_| DbError::ContextError)??
        .into_iter()
        .collect();

        Ok(Response::new(UserCreateResponse { users }))
    }
}

async fn validate_create(conn: &PgPool, names: Arc<[String]>) -> Result<(), ClientError> {
    Validation::new(names, "users", "name")
        .unique_constraint()
        .validate(conn)
        .await
}

async fn user_create(conn: &PgPool, names: Arc<[String]>) -> Result<Vec<UserPgRow>, DbError> {
    // sql statement
    let mut q = sqlx::QueryBuilder::<Postgres>::new("INSERT INTO users (name) ");
    q.push_values(names.iter().take(BIND_LIMIT), |mut b, name| {
        b.push_bind(name);
    });
    q.push(" RETURNING *");
    // transaction
    let mut tx = conn.begin().await?;
    let rows = q.build_query_as::<UserPgRow>().fetch_all(&mut *tx).await?;
    tx.commit().await?;

    Ok(rows)
}

#[cfg(test)]
mod test {
    use crate::errors::expected_error;

    use super::*;

    #[sqlx::test]
    async fn test_user_create(conn: PgPool) -> anyhow::Result<()> {
        let want: String = "jon".into();
        let got = user_create(&conn, vec![want.clone()].into()).await?;

        assert_eq!(want, got.first().unwrap().name);

        Ok(())
    }

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    /// Tests if the unique contraint of the DB is properly enforced,
    /// and the correct error type is returned.
    async fn test_unique_args(conn: PgPool) {
        let want = ClientError::UniqueConstraint("users".into(), "jon".into());
        let names = vec!["ringo".into(), "jon".into()].into();
        let got = validate_create(&conn, names).await.map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }
}
