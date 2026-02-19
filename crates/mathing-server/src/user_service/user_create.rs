use sqlx::{PgPool, Postgres};

use crate::get_duplicates::get_duplicates;

use super::{user_row::UserPgRow, *};

impl MathingUserService {
    pub(super) async fn handle_create(
        &self,
        req: Request<UserCreateRequest>,
    ) -> Result<Response<UserCreateResponse>, Status> {
        let req = req.into_inner();
        info!("{:?}", req);

        let conn = DBconn::try_get().await?;
        let names = req.names.into();

        let users = tokio::time::timeout(DBconn::context(), user_create(conn, names))
            .await
            .map_err(|_| DbError::ContextError)??
            .into_iter()
            .collect();

        Ok(Response::new(UserCreateResponse { users }))
    }
}

async fn user_create(conn: &PgPool, names: Arc<[String]>) -> Result<Vec<UserPgRow>, DbError> {
    // validate names
    if names.is_empty() {
        return Err(DbError::EmptyArgs);
    }
    if let Some(found) = get_duplicates(names.clone()) {
        return Err(DbError::UniqueConstraint("users", found));
    }
    let mut q = sqlx::QueryBuilder::<Postgres>::new("SELECT * FROM users WHERE name IN ");
    q.push_tuples(names.iter().take(BIND_LIMIT), |mut b, name| {
        b.push_bind(name);
    });
    if let Ok(found) = q.build_query_as::<UserPgRow>().fetch_all(conn).await {
        match found.is_empty() {
            true => (),
            false => {
                let found = found
                    .into_iter()
                    .map(|f| f.name)
                    .collect::<Vec<String>>()
                    .join(", ");
                return Err(DbError::UniqueConstraint("users", found));
            }
        }
    };
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
    async fn test_unique_args(conn: PgPool) -> anyhow::Result<()> {
        let want = DbError::UniqueConstraint("users", "jon".into());
        let names = vec!["ringo".into(), "jon".into()].into();
        let got = user_create(&conn, names).await.map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }

    #[sqlx::test]
    async fn test_repeated_args(conn: PgPool) -> anyhow::Result<()> {
        let want = DbError::UniqueConstraint("users", "jon".into());
        let names = vec!["jon".into(); 3].into();
        let got = user_create(&conn, names).await.map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }
}
