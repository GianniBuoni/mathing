use sqlx::PgPool;

use super::{user_row::UserPgRow, *};

impl MathingUserService {
    pub(super) async fn handle_get(
        &self,
        req: Request<UserGetRequest>,
    ) -> Result<Response<UserGetResponse>, Status> {
        let name = req.into_inner();
        info!("{:?}", name);

        let conn = DBconn::try_get().await?;

        let user = tokio::time::timeout(DBconn::context(), user_get(conn, &name.name))
            .await
            .map_err(|_| DbError::ContextError)?
            .map(|u| Some(u.into()))?;

        Ok(Response::new(UserGetResponse { user }))
    }
}

pub(super) async fn user_get(conn: &PgPool, name: &str) -> Result<UserPgRow, DbError> {
    sqlx::query_as!(UserPgRow, "SELECT * FROM users WHERE name=$1", name)
        .fetch_one(conn)
        .await
        .map_err(|_| DbError::EntryNotFound("users", name.to_string()))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[sqlx::test]
    /// Basic get test; expects query to succeed and return the name inputed at the beginning of the test
    async fn test_user_get(conn: PgPool) -> anyhow::Result<()> {
        let want: Arc<str> = "jon".into();
        let now = chrono::Local::now();

        sqlx::query!(
            "INSERT INTO users (created_at, updated_at, name) VALUES ($1, $1, $2)",
            now,
            want.as_ref(),
        )
        .execute(&conn)
        .await?;

        let got = user_get(&conn, want.as_ref()).await?;
        assert_eq!(want, got.name);

        Ok(())
    }

    #[sqlx::test]
    /// Basic error test; expects query to fail and return the correct DB error
    async fn test_user_get_error(conn: PgPool) -> anyhow::Result<()> {
        let name = "jon";
        let want = DbError::EntryNotFound("users", name.into());

        let got = user_get(&conn, name).await.map(|res| {
            let message = format!("Test query expected to fail but returned {:?}", res);
            anyhow::Error::msg(message)
        });

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }

        Ok(())
    }
}
