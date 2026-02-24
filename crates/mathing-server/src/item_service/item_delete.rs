use sqlx::{PgPool, Postgres, QueryBuilder, types::Uuid};

use crate::prelude::mathing_proto::RowsAffected;

use super::*;

impl MathingItemService {
    pub(super) async fn handle_delete(
        &self,
        req: Request<ItemDeleteRequest>,
    ) -> Result<Response<ItemDeleteResponse>, Status> {
        let req = req.into_inner();
        info!("{req:?}");

        let conn = DBconn::try_get().await?;
        let args = Arc::<[String]>::from(req.targets);

        let rows_affected = tokio::time::timeout(DBconn::context(), async {
            validate_delete(conn, args.clone()).await?;
            Ok::<u64, Status>(item_delete(conn, args).await?)
        })
        .await
        .map_err(|_| DbError::ContextError)?
        .map(|rows_affected| RowsAffected { rows_affected })
        .map(Some)?;

        Ok(Response::new(ItemDeleteResponse { rows_affected }))
    }
}

async fn validate_delete(conn: &PgPool, args: Arc<[String]>) -> Result<(), ClientError> {
    Validation::new(args, "items", "uuid")
        .with_uuid_args()
        .with_existant_args()
        .validate(conn)
        .await
}

async fn item_delete(conn: &PgPool, args: Arc<[String]>) -> Result<u64, DbError> {
    // collect uuids
    let args = args
        .iter()
        .map(|f| Uuid::try_parse(f).expect("Uuid's should've already been validtated."))
        .collect::<Vec<Uuid>>();
    // sql statement
    let mut q = QueryBuilder::<Postgres>::new("DELETE from items WHERE uuid IN ");
    q.push_tuples(args, |mut b, uuid| {
        b.push_bind(uuid);
    });
    // transaction
    let mut tx = conn.begin().await?;
    let rows_affected = q.build().execute(&mut *tx).await?.rows_affected();
    tx.commit().await?;

    Ok(rows_affected)
}

#[cfg(test)]
mod tests {
    use sqlx::types::Uuid;

    use crate::errors::expected_error;

    use super::*;

    #[sqlx::test(fixtures("../../fixtures/items.sql"))]
    async fn test_delete(conn: PgPool) -> anyhow::Result<()> {
        let want = 2_u64;

        let args = sqlx::query_scalar!("SELECT uuid FROM items WHERE name IN ('Orange', 'Papaya')")
            .fetch_all(&conn)
            .await?;
        let args = args.iter().map(|f| f.to_string()).collect();
        let got = item_delete(&conn, args).await?;

        assert_eq!(want, got);
        Ok(())
    }

    #[sqlx::test(fixtures("../../fixtures/items.sql"))]
    async fn test_validation_not_found(conn: PgPool) {
        let bad_uuid = Uuid::nil().to_string();
        let want = ClientError::EntryNotFound("items".into(), bad_uuid.clone());
        let got = validate_delete(&conn, Arc::<[String]>::from(vec![bad_uuid]))
            .await
            .map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }

    #[sqlx::test]
    async fn test_validation_parsing(conn: PgPool) {
        let bad_uuid = vec!["bob".to_string(), "orange".to_string()];
        let want = ClientError::Uuid(bad_uuid.join(", "));
        let got = validate_delete(&conn, bad_uuid.into())
            .await
            .map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }
}
