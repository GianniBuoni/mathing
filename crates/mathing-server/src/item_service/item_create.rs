use sqlx::{PgPool, Postgres};

use crate::prelude::mathing_proto::{ItemCreate, ItemRow};

use super::{item_row::ItemPgRow, *};

impl MathingItemService {
    pub(super) async fn handle_create(
        &self,
        req: Request<ItemCreateRequest>,
    ) -> Result<Response<ItemCreateResponse>, Status> {
        let req = req.into_inner();
        info!("{req:?}");

        let conn = DBconn::try_get().await?;
        let args = Arc::<[ItemCreate]>::from(req.items);

        let items = tokio::time::timeout(DBconn::context(), async {
            validate_create(conn, args.clone()).await?;
            Ok::<Vec<ItemPgRow>, Status>(item_create(conn, args).await?)
        })
        .await
        .map_err(|_| DbError::ContextError)??
        .into_iter()
        .map(ItemRow::try_from)
        .collect::<Result<Vec<ItemRow>, ServerError>>()?;

        Ok(Response::new(ItemCreateResponse { items }))
    }
}

async fn validate_create(conn: &PgPool, args: Arc<[ItemCreate]>) -> Result<(), ClientError> {
    let args = args
        .iter()
        .cloned()
        .map(|f| f.name)
        .collect::<Arc<[String]>>();

    Validation::new(args, "items", "name")
        .unique_constraint()
        .validate(conn)
        .await
}

async fn item_create(conn: &PgPool, args: Arc<[ItemCreate]>) -> Result<Vec<ItemPgRow>, DbError> {
    // sql statement
    let mut q = sqlx::QueryBuilder::<Postgres>::new("INSERT INTO items (name, price) ");
    q.push_values(args.iter().take(BIND_LIMIT / 2), |mut b, item| {
        b.push_bind(&item.name);
        b.push_bind(item.price);
    });
    q.push(" RETURNING *");
    // transaction
    let mut tx = conn.begin().await?;
    let rows = q.build_query_as::<ItemPgRow>().fetch_all(&mut *tx).await?;
    tx.commit().await?;

    Ok(rows)
}

#[cfg(test)]
mod tests {
    use crate::errors::expected_error;

    use super::*;

    fn salmon() -> ItemCreate {
        ItemCreate {
            name: "salmon".into(),
            price: 14.94,
        }
    }

    fn chicken() -> ItemCreate {
        ItemCreate {
            name: "chicken".into(),
            price: 9.99,
        }
    }

    #[sqlx::test]
    async fn test_item_create(conn: PgPool) -> anyhow::Result<()> {
        let want = "salmon".to_string();
        let got = item_create(&conn, vec![salmon()].into()).await?;

        assert_eq!(want, got.first().unwrap().name);
        Ok(())
    }

    #[sqlx::test(fixtures("../../fixtures/items.sql"))]
    // duplicate item names should not be allowed
    async fn test_validation(conn: PgPool) {
        let want = ClientError::UniqueConstraint("items".into(), "salmon".into());
        let args = [salmon(), chicken()].into();
        let got = validate_create(&conn, args).await.map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }
}
