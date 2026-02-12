use sqlx::types::Uuid;

use crate::prelude::mathing_proto::{RowsAffected, one_of_id::OneOfId};

use super::*;

impl MathingUserService {
    pub(super) async fn handle_delete(
        &self,
        req: Request<UserDeleteRequest>,
    ) -> Result<Response<UserDeleteResponse>, Status> {
        let req = req.into_inner();
        info!("{:?}", req);

        let id = req
            .one_of_id
            .ok_or(ClientError::MissingField("name or uuid"))?
            .one_of_id
            .ok_or(ClientError::MissingField("name or uuid"))?;

        let conn = DBconn::try_get().await?;
        let (mut ctx, _handle) = DBconn::context();

        let rows_affected = tokio::select! {
            _ = ctx.done() => return Err(
                DbError::ContextError.into()
            ),
            rows = user_delete(conn, id) => {
                let rows_affected = rows?;
                Some(RowsAffected { rows_affected })
            }
        };

        Ok(Response::new(UserDeleteResponse { rows_affected }))
    }
}

async fn user_delete(conn: &sqlx::PgPool, one_of_id: OneOfId) -> Result<u64, DbError> {
    match one_of_id {
        OneOfId::Name(s) => user_delete_name(conn, s.as_str()).await,
        OneOfId::Uuid(u) => user_delete_uuid(conn, u.parse().map_err(|_| DbError::Uuid(u))?).await,
    }
}

async fn user_delete_name(conn: &sqlx::PgPool, name: &str) -> Result<u64, DbError> {
    // check if user name exists
    let rows = super::user_get::user_get(conn, name).await?;
    user_delete_uuid(conn, rows.uuid).await
}

async fn user_delete_uuid(conn: &sqlx::PgPool, uuid: Uuid) -> Result<u64, DbError> {
    let mut tx = conn.begin().await?;

    let rows = sqlx::query!("DELETE FROM users WHERE uuid=$1", uuid)
        .execute(&mut *tx)
        .await?
        .rows_affected();

    tx.commit().await?;
    Ok(rows)
}
