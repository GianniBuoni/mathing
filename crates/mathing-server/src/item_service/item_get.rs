use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{item_service::item_row::ItemPgRow, prelude::mathing_proto::ItemRow};

use super::*;

impl MathingItemService {
    pub(super) async fn handle_get(
        &self,
        req: Request<ItemGetRequest>,
    ) -> Result<Response<ItemGetResponse>, Status> {
        let req = req.into_inner();
        info!("{req:?}");

        let conn = DBconn::try_get().await?;
        let args = Arc::<[String]>::from(req.targets);
        let mut paginaiton = PaginationBuilder::from(req.pagination);

        let (pagination, items) = tokio::time::timeout(DBconn::context(), async {
            validate_args(conn, args.clone()).await?;
            let count = item_count(conn, args.clone()).await?;
            paginaiton.with_count(count);
            let pagination = paginaiton.try_build()?;
            pagination.try_validate()?;
            let items = item_get(conn, args, pagination).await?;
            Ok::<(Pagination, Vec<ItemPgRow>), Status>((pagination, items))
        })
        .await
        .map_err(|_| DbError::ContextError)??;

        let items = items
            .into_iter()
            .map(|f| ItemRow::try_from(f))
            .collect::<Result<Vec<ItemRow>, ServerError>>()?;

        Ok(Response::new(ItemGetResponse {
            items,
            pagination: Some(pagination.into()),
        }))
    }
}

async fn validate_args(conn: &PgPool, args: Arc<[String]>) -> Result<(), ClientError> {
    Validation::new(args, "items", "name")
        .with_existant_args()
        .with_uuid_args()
        .validate(conn)
        .await
}

async fn item_count(conn: &PgPool, args: Arc<[String]>) -> Result<u32, Status> {
    let mut q = QueryBuilder::<Postgres>::new("SELECT COUNT(*) AS rows FROM items WHERE name IN (");
    args.iter()
        .take(BIND_LIMIT / args.len())
        .enumerate()
        .for_each(|(i, arg)| {
            if i > 0 {
                q.push(" UNION ");
            }
            q.push("SELECT name FROM items WHERE name LIKE concat('%', ");
            q.push_bind(arg);
            q.push(", '%')");
        });
    q.push(")");
    // transaction
    let count = q
        .build_query_scalar::<i64>()
        .fetch_one(conn)
        .await
        .map_err(|e| DbError::from(e))?;

    let count = u32::try_from(count)
        .map_err(|_| ServerError::ConversionError("i64", "u32", count.to_string()))?;
    Ok(count)
}

async fn item_get(
    conn: &PgPool,
    args: Arc<[String]>,
    page: Pagination,
) -> Result<Vec<ItemPgRow>, DbError> {
    // sql statement
    let mut q = QueryBuilder::<Postgres>::new("SELECT * FROM items WHERE name IN (");
    args.iter()
        .take(BIND_LIMIT / args.len())
        .enumerate()
        .for_each(|(i, arg)| {
            if i > 0 {
                q.push_bind(" UNION ");
            }
            q.push("SELECT name FROM items WHERE name LIKE concat('%', ");
            q.push_bind(arg);
            q.push(", '%')");
        });
    q.push(") ORDER BY name LIMIT ")
        .push_bind(i64::from(page.limit))
        .push(" OFFSET ")
        .push_bind(i64::from(page.get_offset()));
    // transaction
    Ok(q.build_query_as::<ItemPgRow>().fetch_all(conn).await?)
}

#[cfg(test)]
mod tests {
    use sqlx::types::Uuid;

    use crate::{errors::expected_error, prelude::mathing_proto::PaginationRequest};

    use super::*;

    #[sqlx::test(fixtures("../../fixtures/items.sql"))]
    async fn test_item_get(conn: PgPool) -> anyhow::Result<()> {
        let expected_length = 2;
        let want = vec!["Elderberry".to_string(), "Gooseberry".into()];

        let args = Arc::<[String]>::from(["berry".into()]);
        let count = item_count(&conn, args.clone()).await?;
        let mut pagination = PaginationBuilder::from(Some(PaginationRequest { limit: 2, page: 2 }));
        pagination.with_count(count);
        let pagination = pagination.try_build()?;
        pagination.try_validate()?;

        let got = item_get(&conn, args, pagination).await?;

        assert_eq!(expected_length, got.len());
        got.into_iter()
            .map(|f| f.name)
            .zip(want)
            .for_each(|(got, want)| {
                assert_eq!(want, got);
            });
        Ok(())
    }

    #[sqlx::test(fixtures("../../fixtures/items.sql"))]
    /// the more args provided the more matches should be added
    async fn test_item_count(conn: PgPool) -> anyhow::Result<()> {
        let want = 8_u32; // "oo" + "berry";
        let args = Arc::<[String]>::from(["oo".into(), "berry".into()]);
        let got = item_count(&conn, args).await?;

        assert_eq!(want, got);
        Ok(())
    }

    #[sqlx::test]
    /// Test if client arguments are all unique/not-repeated
    async fn test_target_validation(conn: PgPool) {
        let args = Arc::<[String]>::from(vec![Uuid::nil().to_string(); 3]);
        let want = ClientError::RpeatedValue(args.first().unwrap().clone());
        let got = validate_args(&conn, args).await.map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }
}
