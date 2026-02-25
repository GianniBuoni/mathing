use sqlx::PgPool;

use crate::{
    item_service::item_row::ItemPgRow,
    prelude::mathing_proto::{ItemRow, PaginationResponse},
};

use super::*;

impl MathingItemService {
    pub(super) async fn handle_list(
        &self,
        req: Request<ItemListRequest>,
    ) -> Result<Response<ItemListResponse>, Status> {
        let req = req.into_inner();
        info!("{req:?}");

        let conn = DBconn::try_get().await?;
        let mut offset = OffsetBuilder::from(req.pagination);

        let (total_rows, items) = tokio::time::timeout(DBconn::context(), async {
            let count = item_count(conn).await?;
            offset.with_count(count);
            let offset = offset.try_build()?;
            offset.validate()?;
            let items = item_list(conn, offset).await?;
            Ok::<(u32, Vec<ItemPgRow>), Status>((count, items))
        })
        .await
        .map_err(|_| DbError::ContextError)??;

        let items = items
            .into_iter()
            .map(|f| ItemRow::try_from(f))
            .collect::<Result<Vec<ItemRow>, ServerError>>()?;

        Ok(Response::new(ItemListResponse {
            items,
            pagination: Some(PaginationResponse {
                total_rows,
                total_pages: 0,
            }),
        }))
    }
}

async fn item_count(conn: &PgPool) -> Result<u32, Status> {
    let res = sqlx::query_scalar!("SELECT COUNT(*) AS rows FROM items")
        .fetch_one(conn)
        .await
        .map_err(DbError::from)?
        .unwrap_or_default();
    let res = u32::try_from(res)
        .map_err(|_| ServerError::ConversionError("i64", "u32", res.to_string()))?;

    Ok(res)
}

async fn item_list(conn: &PgPool, offset: Offset) -> Result<Vec<ItemPgRow>, DbError> {
    let res = sqlx::query_as!(
        ItemPgRow,
        "SELECT * FROM items ORDER BY name LIMIT $1 OFFSET $2",
        i64::from(offset.limit),
        i64::from(offset.get_offset()),
    )
    .fetch_all(conn)
    .await?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::prelude::mathing_proto::PaginationRequest;

    use super::*;

    #[sqlx::test(fixtures("../../fixtures/items.sql"))]
    async fn test_count(conn: PgPool) -> anyhow::Result<()> {
        let want = 43_u32;
        let got = item_count(&conn).await?;

        assert_eq!(want, got);
        Ok(())
    }

    #[sqlx::test(fixtures("../../fixtures/items.sql"))]
    async fn test_list(conn: PgPool) -> anyhow::Result<()> {
        let want_len = 20;
        let want_fist = "Açaí";

        let offset = PaginationRequest { page: 1, limit: 20 };
        let mut offset = OffsetBuilder::from(Some(offset));
        let count = item_count(&conn).await?;
        offset.with_count(count);
        let offset = offset.try_build()?;
        offset.validate()?;

        let got = item_list(&conn, offset).await?;

        assert_eq!(want_len, got.len());
        assert_eq!(want_fist, &got.first().unwrap().name);
        Ok(())
    }

    #[sqlx::test(fixtures("../../fixtures/items.sql"))]
    async fn test_offset(conn: PgPool) -> anyhow::Result<()> {
        let want_len = 3;
        let want_first = "Strawberry";

        let offset = PaginationRequest { page: 5, limit: 10 };
        let mut offset = OffsetBuilder::from(Some(offset));
        let count = item_count(&conn).await?;
        offset.with_count(count);
        let offset = offset.try_build()?;
        offset.validate()?;

        let got = item_list(&conn, offset).await?;

        assert_eq!(want_len, got.len());
        assert_eq!(want_first, &got.first().unwrap().name);
        Ok(())
    }
}
