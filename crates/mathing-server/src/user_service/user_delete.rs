use sqlx::{PgPool, types::Uuid};

use crate::prelude::mathing_proto::{RowsAffected, one_of_id::OneOfId};

use super::*;

impl MathingUserService {
    pub(super) async fn handle_delete(
        &self,
        req: Request<UserDeleteRequest>,
    ) -> Result<Response<UserDeleteResponse>, Status> {
        let req = req.into_inner();
        info!("{:?}", req);

        let one_of_id = req
            .one_of_id
            .ok_or(ClientError::MissingField("name or uuid"))?
            .one_of_id
            .ok_or(ClientError::MissingField("name or uuid"))?;

        let conn = DBconn::try_get().await?;

        let rows_affected = tokio::time::timeout(DBconn::context(), user_delete(conn, one_of_id))
            .await
            .map_err(|_| DbError::ContextError)?
            .map(|rows_affected| Some(RowsAffected { rows_affected }))?;

        Ok(Response::new(UserDeleteResponse { rows_affected }))
    }
}

async fn user_delete(conn: &PgPool, one_of_id: OneOfId) -> Result<u64, DbError> {
    match one_of_id {
        OneOfId::Name(s) => user_delete_name(conn, s.as_str()).await,
        OneOfId::Uuid(u) => user_delete_uuid(conn, u.parse().map_err(|_| DbError::Uuid(u))?).await,
    }
}

async fn user_delete_name(conn: &PgPool, name: &str) -> Result<u64, DbError> {
    // check if user name exists
    let rows = super::user_get::user_get(conn, name).await?;
    user_delete_uuid(conn, rows.uuid).await
}

async fn user_delete_uuid(conn: &PgPool, uuid: Uuid) -> Result<u64, DbError> {
    let mut tx = conn.begin().await?;
    // check if uuid exists
    let _ = sqlx::query!("SELECT * FROM users WHERE uuid=$1", uuid)
        .fetch_one(conn)
        .await
        .map_err(|_| DbError::EntryNotFound("users", uuid.to_string()))?;

    let rows = sqlx::query!("DELETE FROM users WHERE uuid=$1", uuid)
        .execute(&mut *tx)
        .await?
        .rows_affected();

    tx.commit().await?;
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::{user_row::UserPgRow, *};

    async fn get_uuid(conn: &PgPool, name: &str) -> Result<Uuid, sqlx::Error> {
        Ok(
            sqlx::query_as!(UserPgRow, "SELECT * FROM users WHERE name=$1", name)
                .fetch_one(conn)
                .await?
                .uuid,
        )
    }

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    async fn test_user_delete_name(conn: PgPool) -> anyhow::Result<()> {
        let want = 1;
        let got = user_delete_name(&conn, "jon").await?;

        assert_eq!(want, got);
        Ok(())
    }

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    async fn test_user_delete_uuid(conn: PgPool) -> anyhow::Result<()> {
        let want = 1;
        let uuid = get_uuid(&conn, "jon").await?;
        let got = user_delete_uuid(&conn, uuid).await?;

        assert_eq!(want, got);
        Ok(())
    }

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    async fn test_user_delete_one_of_name(conn: PgPool) -> anyhow::Result<()> {
        let want = 1;
        let got = user_delete(&conn, OneOfId::Name("jon".into())).await?;

        assert_eq!(want, got);
        Ok(())
    }

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    async fn test_user_delete_one_of_uuid(conn: PgPool) -> anyhow::Result<()> {
        let want = 1;
        let uuid = get_uuid(&conn, "noodle").await?;
        let got = user_delete(&conn, OneOfId::Uuid(uuid.to_string())).await?;

        assert_eq!(want, got);
        Ok(())
    }

    #[sqlx::test]
    async fn test_user_delete_error_name(conn: PgPool) -> anyhow::Result<()> {
        let want = DbError::EntryNotFound("users", "jon".into());
        let got = user_delete(&conn, OneOfId::Name("jon".into()))
            .await
            .map(|u| {
                let message = format!("Test expected an error but returned: {u:?}");
                anyhow::Error::msg(message)
            });

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }

    #[sqlx::test]
    async fn test_user_delete_error_uuid(conn: PgPool) -> anyhow::Result<()> {
        let uuid = Uuid::nil();
        let want = DbError::EntryNotFound("users", uuid.to_string());
        let got = user_delete(&conn, OneOfId::Uuid(uuid.to_string()))
            .await
            .map(|u| {
                let message = format!("Test expected an error, but returned: {u:?}");
                anyhow::Error::msg(message)
            });

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }
}
