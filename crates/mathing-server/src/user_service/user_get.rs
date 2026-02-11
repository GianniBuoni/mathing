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

        let user = Some(
            user_get(conn, name.name.as_str())
                .await
                .map_err(|e| Status::internal(e.to_string()))?
                .into(),
        );

        Ok(Response::new(UserGetResponse { user }))
    }
}

pub(super) async fn user_get(conn: &PgPool, name: &str) -> anyhow::Result<UserPgRow> {
    sqlx::query_as!(UserPgRow, "SELECT * FROM users WHERE name=$1", name)
        .fetch_one(conn)
        .await
        .map_err(|_| {
            let message =
                format!("Db could not find a user entry matching the given name: '{name}'",);
            anyhow::Error::msg(message)
        })
}
