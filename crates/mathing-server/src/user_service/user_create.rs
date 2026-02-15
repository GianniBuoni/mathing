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
