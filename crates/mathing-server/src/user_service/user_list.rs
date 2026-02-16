use sqlx::PgPool;

use crate::prelude::mathing_proto::UserRow;

use super::{user_row::UserPgRow, *};

impl MathingUserService {
    pub(super) async fn handle_list(
        &self,
        req: Request<UserListRequest>,
    ) -> Result<Response<UserListResponse>, Status> {
        info!("{:?}", req.into_inner());

        let conn = DBconn::try_get().await?;

        let users = tokio::time::timeout(DBconn::context(), user_list(conn))
            .await
            .map_err(|_| DbError::ContextError)??
            .into_iter()
            .map(Into::<UserRow>::into)
            .collect();

        Ok(Response::new(UserListResponse { users }))
    }
}

async fn user_list(conn: &PgPool) -> Result<Vec<UserPgRow>, DbError> {
    Ok(sqlx::query_as!(UserPgRow, "SELECT * FROM users")
        .fetch_all(conn)
        .await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    /// Basic list test, checks that given a table with entries,
    /// the expected amount of entries are retrieved.
    async fn test_user_list(conn: PgPool) -> anyhow::Result<()> {
        let want = 3;
        let got = user_list(&conn).await?.len();

        assert_eq!(want, got);

        Ok(())
    }

    #[sqlx::test]
    /// A list request should not error out on an empty table,
    /// but instead return and empty Vec.
    async fn test_user_list_empty(conn: PgPool) -> anyhow::Result<()> {
        let got = user_list(&conn).await?;
        assert!(got.is_empty());

        Ok(())
    }
}
