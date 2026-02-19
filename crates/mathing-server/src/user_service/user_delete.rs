use sqlx::{PgPool, Postgres};

use crate::prelude::mathing_proto::RowsAffected;

use super::*;

impl MathingUserService {
    pub(super) async fn handle_delete(
        &self,
        req: Request<UserDeleteRequest>,
    ) -> Result<Response<UserDeleteResponse>, Status> {
        let req = req.into_inner();
        info!("{:?}", req);

        let names = req.targets.into();
        let conn = DBconn::try_get().await?;

        let rows_affected = tokio::time::timeout(DBconn::context(), user_delete(conn, names))
            .await
            .map_err(|_| DbError::ContextError)?
            .map(|rows_affected| Some(RowsAffected { rows_affected }))?;

        Ok(Response::new(UserDeleteResponse { rows_affected }))
    }
}

async fn user_delete(conn: &PgPool, names: Arc<[String]>) -> Result<u64, DbError> {
    // validate names
    let _ = user_get::user_get(conn, names.clone()).await?;
    // sql statement
    let mut q = sqlx::QueryBuilder::<Postgres>::new("DELETE FROM users WHERE name IN ");
    q.push_tuples(names.iter().take(BIND_LIMIT), |mut b, name| {
        b.push_bind(name);
    });
    // transaction
    let mut tx = conn.begin().await?;
    let rows = q.build().execute(&mut *tx).await?.rows_affected();
    tx.commit().await?;
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use crate::errors::expected_error;

    use super::*;

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    async fn test_user_delete(conn: PgPool) -> anyhow::Result<()> {
        let want = 2;
        let names = vec!["jon".into(), "blue".into()].into();
        let got = user_delete(&conn, names).await?;

        assert_eq!(want, got);
        Ok(())
    }

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    /// Client can successfully send duplicate requests to delete the same user.
    /// To guard against this behavior, an error should be returned.
    async fn test_repeated_args(conn: PgPool) -> anyhow::Result<()> {
        let want = DbError::UniqueConstraint("users", "jon".into());
        let names = vec!["jon".into(); 3].into();
        let got = user_delete(&conn, names).await.map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }

    #[sqlx::test]
    async fn test_user_delete_error(conn: PgPool) -> anyhow::Result<()> {
        let want = DbError::EntryNotFound("users", "jon".into());

        let got = user_delete(&conn, vec!["jon".into()].into())
            .await
            .map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }
}
