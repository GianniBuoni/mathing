use crate::prelude::mathing_proto::UserRow;

use super::{user_row::UserPgRow, *};

impl MathingUserService {
    pub(super) async fn handle_create(
        &self,
        req: Request<UserCreateRequest>,
    ) -> Result<Response<UserCreateResponse>, Status> {
        let req = req.into_inner();

        info!("{:?}", req);
        let conn = DBconn::try_get().await?;
        let (mut ctx, _handle) = DBconn::context();

        let user_row = tokio::select! {
            _ = ctx.done() => return Err(DbError::ContextError.into()),
            user = user_create(conn, req.name.as_str()) => Some(
                user.map(Into::<UserRow>::into)?
            ),
        };

        Ok(Response::new(UserCreateResponse { user_row }))
    }
}

async fn user_create(conn: &sqlx::PgPool, name: &str) -> Result<UserPgRow, DbError> {
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
