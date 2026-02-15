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
