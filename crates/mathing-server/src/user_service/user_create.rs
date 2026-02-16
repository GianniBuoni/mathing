use sqlx::PgPool;

use super::{user_row::UserPgRow, *};

impl MathingUserService {
    pub(super) async fn handle_create(
        &self,
        req: Request<UserCreateRequest>,
    ) -> Result<Response<UserCreateResponse>, Status> {
        let req = req.into_inner();

        info!("{:?}", req);
        let conn = DBconn::try_get().await?;

        let user_row = tokio::time::timeout(DBconn::context(), user_create(conn, &req.name))
            .await
            .map_err(|_| DbError::ContextError)?
            .map(|u| Some(u.into()))?;

        Ok(Response::new(UserCreateResponse { user_row }))
    }
}

async fn user_create(conn: &PgPool, name: &str) -> Result<UserPgRow, DbError> {
    let mut tx = conn.begin().await?;
    let now = chrono::Local::now();

    let row = sqlx::query_as!(
        UserPgRow,
        "
        INSERT INTO users (
            created_at, updated_at, name
        ) VALUES (
            $1, $2, $3
        ) RETURNING *;
        ",
        now,
        now,
        name,
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| DbError::UniqueConstraint("users", "name"))?;

    tx.commit().await?;
    Ok(row)
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use super::*;

    #[sqlx::test]
    async fn test_user_create(conn: PgPool) -> anyhow::Result<()> {
        let want: Arc<str> = "jon".into();
        let got = user_create(&conn, "jon").await?;

        assert_eq!(want, got.name);

        Ok(())
    }

    #[sqlx::test]
    /// Tests if the unique contraint of the DB is properly enforced,
    /// and the correct error type is returned.
    async fn test_user_create_unique(conn: PgPool) -> anyhow::Result<()> {
        let name = "jon";
        let want = DbError::UniqueConstraint("users", "name");

        let _ = user_create(&conn, name).await?;
        let got = user_create(&conn, name).await.map(|u| {
            let message = format!("Test expected an error, but returned {:?}", u);
            anyhow::Error::msg(message)
        });

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }

        Ok(())
    }
}
