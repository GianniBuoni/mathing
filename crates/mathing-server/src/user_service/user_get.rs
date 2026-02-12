use sqlx::PgPool;

use crate::prelude::mathing_proto::UserRow;

use super::{user_row::UserPgRow, *};

impl MathingUserService {
    pub(super) async fn handle_get(
        &self,
        req: Request<UserGetRequest>,
    ) -> Result<Response<UserGetResponse>, Status> {
        let name = req.into_inner();
        info!("{:?}", name);

        let conn = DBconn::try_get().await?;
        let (mut ctx, _handle) = DBconn::context();

        let user = tokio::select! {
            _ = ctx.done() => return Err(
                DbError::ContextError.into()
            ),
            user = user_get(conn, name.name.as_str()) => Some(
                user
                .map(Into::<UserRow>::into)?
            ),
        };

        Ok(Response::new(UserGetResponse { user }))
    }
}

pub(super) async fn user_get(conn: &PgPool, name: &str) -> Result<UserPgRow, DbError> {
    sqlx::query_as!(UserPgRow, "SELECT * FROM users WHERE name=$1", name)
        .fetch_one(conn)
        .await
        .map_err(|_| DbError::EntryNotFound("users", name.to_string()))
}
