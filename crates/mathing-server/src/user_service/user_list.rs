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

        let users = user_list(conn)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .into_iter()
            .map(Into::<UserRow>::into)
            .collect();

        Ok(Response::new(UserListResponse { users }))
    }
}

async fn user_list(conn: &PgPool) -> anyhow::Result<Vec<UserPgRow>> {
    Ok(sqlx::query_as!(UserPgRow, "SELECT * FROM users")
        .fetch_all(conn)
        .await?)
}
