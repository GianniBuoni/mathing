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
            .ok_or(Status::invalid_argument("No deletion fields set."))?
            .one_of_id
            .ok_or(Status::invalid_argument("No deletion fields set."))?;

        let conn = DBconn::try_get().await?;

        let rows_affected = user_delete(conn, id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let message = UserDeleteResponse {
            rows_affected: Some(RowsAffected { rows_affected }),
        };

        Ok(Response::new(message))
    }
}

async fn user_delete(conn: &sqlx::PgPool, one_of_id: OneOfId) -> anyhow::Result<u64> {
    match one_of_id {
        OneOfId::Name(s) => user_delete_name(conn, s.as_str()).await,
        OneOfId::Uuid(u) => user_delete_uuid(conn, u.parse()?).await,
    }
}

async fn user_delete_name(conn: &sqlx::PgPool, name: &str) -> anyhow::Result<u64> {
    // check if user name exists
    let rows = super::user_get::user_get(conn, name).await?;
    user_delete_uuid(conn, rows.uuid).await
}

async fn user_delete_uuid(conn: &sqlx::PgPool, uuid: Uuid) -> anyhow::Result<u64> {
    let mut tx = conn.begin().await?;

    let rows = sqlx::query!("DELETE FROM users WHERE uuid=$1", uuid)
        .execute(&mut *tx)
        .await?
        .rows_affected();

    tx.commit().await?;
    Ok(rows)
}
