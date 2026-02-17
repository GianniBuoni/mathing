use sqlx::PgPool;

use crate::prelude::mathing_proto::RowsAffected;

use super::*;

impl MathingUserService {
    pub(super) async fn handle_delete(
        &self,
        req: Request<UserDeleteRequest>,
    ) -> Result<Response<UserDeleteResponse>, Status> {
        let req = req.into_inner();
        info!("{:?}", req);

        let name = req.target;

        let conn = DBconn::try_get().await?;

        let rows_affected =
            tokio::time::timeout(DBconn::context(), user_delete(conn, name.as_ref()))
                .await
                .map_err(|_| DbError::ContextError)?
                .map(|rows_affected| Some(RowsAffected { rows_affected }))?;

        Ok(Response::new(UserDeleteResponse { rows_affected }))
    }
}

async fn user_delete(conn: &PgPool, name: &str) -> Result<u64, DbError> {
    // check if user name exists
    let uuid = super::user_get::user_get(conn, name).await?.uuid;

    let mut tx = conn.begin().await?;
    let rows = sqlx::query!("DELETE FROM users WHERE uuid=$1", uuid)
        .execute(&mut *tx)
        .await?
        .rows_affected();

    tx.commit().await?;
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    async fn test_user_delete(conn: PgPool) -> anyhow::Result<()> {
        let want = 1;
        let got = user_delete(&conn, "jon").await?;

        assert_eq!(want, got);
        Ok(())
    }

    #[sqlx::test]
    async fn test_user_delete_error(conn: PgPool) -> anyhow::Result<()> {
        let want = DbError::EntryNotFound("users", "jon".into());

        let got = user_delete(&conn, "jon").await.map(|u| {
            let message = format!("Test expected an error but returned: {u:?}");
            anyhow::Error::msg(message)
        });

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }
}
