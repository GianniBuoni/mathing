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
