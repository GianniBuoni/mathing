use sqlx::{PgConnection, PgPool, Postgres, QueryBuilder, types::Uuid};

use crate::prelude::mathing_proto::ItemRow;

use super::{item_row::ItemPgRow, mathing_proto::ItemEdit, *};

impl MathingItemService {
    pub(super) async fn handle_edit(
        &self,
        req: Request<ItemEditRequest>,
    ) -> Result<Response<ItemEditResponse>, Status> {
        let req = req.into_inner();
        info!("{req:?}");

        let conn = DBconn::try_get().await?;
        let args = Arc::<[ItemEdit]>::from(req.item_edits);

        let items = tokio::time::timeout(DBconn::context(), async {
            validate_edit(conn, args.clone()).await?;
            Ok::<Vec<ItemPgRow>, Status>(item_edit(conn, args).await?)
        })
        .await
        .map_err(|_| DbError::ContextError)??
        .into_iter()
        .map(ItemRow::try_from)
        .collect::<Result<Vec<ItemRow>, ServerError>>()?;

        Ok(Response::new(ItemEditResponse { items }))
    }
}

async fn validate_edit(conn: &PgPool, args: Arc<[ItemEdit]>) -> Result<(), ClientError> {
    let old = args.iter().map(|f| f.targets.to_owned()).collect();
    Validation::new(old, "items", "uuid")
        .with_existant_args()
        .with_uuid_args()
        .validate(conn)
        .await?;

    let new = args.iter().filter_map(|f| f.name.to_owned()).collect();
    Validation::new(new, "items", "name")
        .with_unique_constraint()
        .validate(conn)
        .await?;

    Ok(())
}

async fn item_edit(conn: &PgPool, args: Arc<[ItemEdit]>) -> Result<Vec<ItemPgRow>, DbError> {
    // filter name: Some(), price: None
    let edit_name = args
        .clone()
        .iter()
        .filter(|f| f.name.is_some() && f.price.is_none())
        .cloned()
        .collect::<Arc<[ItemEdit]>>();
    // filter name: None, price: Some()
    let edit_price = args
        .iter()
        .filter(|f| f.name.is_none() && f.price.is_some())
        .cloned()
        .collect::<Arc<[ItemEdit]>>();
    // // filter name: Some(), price: Some()
    let edit_all = args
        .iter()
        .filter(|f| f.name.is_some() && f.price.is_some())
        .cloned()
        .collect::<Arc<[ItemEdit]>>();
    // transactions
    let mut tx = conn.begin().await?;
    let mut names = match edit_name.is_empty() {
        true => vec![],
        false => name_edit(&mut tx, edit_name).await?,
    };
    let mut prices = match edit_price.is_empty() {
        true => vec![],
        false => price_edit(&mut tx, edit_price).await?,
    };
    let mut res = match edit_all.is_empty() {
        true => vec![],
        false => all_edit(&mut tx, edit_all).await?,
    };
    tx.commit().await?;

    res.append(&mut names);
    res.append(&mut prices);
    Ok(res)
}

async fn name_edit(
    tx: &mut PgConnection,
    args: Arc<[ItemEdit]>,
) -> Result<Vec<ItemPgRow>, DbError> {
    let mut q = QueryBuilder::<Postgres>::new(
        "UPDATE items SET name = data.new_name, updated_at = CURRENT_TIMESTAMP FROM (",
    );
    q.push_values(args.iter().take(BIND_LIMIT / 2), |mut b, item| {
        let uuid =
            Uuid::try_parse(&item.targets).expect("UUID's should've already been validated.");
        b.push_bind(uuid);
        b.push_bind(&item.name);
    });
    q.push(") AS data(uuid, new_name) WHERE items.uuid = data.uuid RETURNING *");
    Ok(q.build_query_as::<ItemPgRow>().fetch_all(&mut *tx).await?)
}

async fn price_edit(
    tx: &mut PgConnection,
    args: Arc<[ItemEdit]>,
) -> Result<Vec<ItemPgRow>, DbError> {
    let mut q = QueryBuilder::<Postgres>::new(
        "UPDATE items SET price = data.new_price, updated_at = CURRENT_TIMESTAMP FROM (",
    );
    q.push_values(args.iter().take(BIND_LIMIT / 2), |mut b, item| {
        let uuid =
            Uuid::try_parse(&item.targets).expect("UUID's should've already been validated.");
        b.push_bind(uuid);
        b.push_bind(item.price);
    });
    q.push(") AS data(uuid, new_price) WHERE items.uuid = data.uuid RETURNING *");
    Ok(q.build_query_as::<ItemPgRow>().fetch_all(&mut *tx).await?)
}

async fn all_edit(tx: &mut PgConnection, args: Arc<[ItemEdit]>) -> Result<Vec<ItemPgRow>, DbError> {
    let mut q = QueryBuilder::<Postgres>::new(
        "UPDATE items SET name = data.new_name, price = data.new_price, updated_at = CURRENT_TIMESTAMP FROM (",
    );
    q.push_values(args.iter().take(BIND_LIMIT / 3), |mut b, item| {
        let uuid =
            Uuid::try_parse(&item.targets).expect("UUID's should've already been validated.");
        b.push_bind(uuid);
        b.push_bind(&item.name);
        b.push_bind(item.price);
    });
    q.push(") AS data(uuid, new_name, new_price) WHERE items.uuid = data.uuid RETURNING *");
    Ok(q.build_query_as::<ItemPgRow>().fetch_all(&mut *tx).await?)
}

#[cfg(test)]
mod tests {
    use crate::errors::expected_error;

    use super::*;

    fn new_names() -> Vec<String> {
        vec!["marmots".to_string(), "acorns".into(), "shoes".into()]
    }
    fn new_prices() -> Vec<f32> {
        vec![1.33, 0.00, 99.99]
    }

    /// Returns 3 valid uuids for testing
    async fn get_valid_uuids(conn: &PgPool) -> Result<Vec<ItemPgRow>, sqlx::Error> {
        sqlx::query_as!(
            ItemPgRow,
            "SELECT * FROM items WHERE name IN ('Lemon', 'Mango', 'Orange')"
        )
        .fetch_all(conn)
        .await
    }

    #[sqlx::test(fixtures("../../fixtures/items.sql"))]
    fn test_edit_name(conn: PgPool) -> anyhow::Result<()> {
        let originals = get_valid_uuids(&conn).await?;
        // form edits
        let args = originals
            .iter()
            .zip(new_names())
            .map(|(old, name)| ItemEdit {
                targets: old.uuid.to_string(),
                name: Some(name),
                price: None,
            })
            .collect();
        let got = item_edit(&conn, args).await?;
        // assert_ne names and assert_eq prices
        originals.iter().zip(got).for_each(|(old, new)| {
            assert_eq!(old.uuid, new.uuid);
            assert_ne!(old.updated_at, new.updated_at);
            assert_ne!(old.name, new.name);
            assert_eq!(old.price, new.price);
        });
        Ok(())
    }

    #[sqlx::test(fixtures("../../fixtures/items.sql"))]
    async fn test_edit_price(conn: PgPool) -> anyhow::Result<()> {
        let originals = get_valid_uuids(&conn).await?;
        // form item edit
        let args = originals
            .iter()
            .zip(new_prices())
            .map(|(old, price)| ItemEdit {
                targets: old.uuid.to_string(),
                name: None,
                price: Some(price),
            })
            .collect();
        let got = item_edit(&conn, args).await?;
        // assertions
        originals.iter().zip(got).for_each(|(old, new)| {
            assert_eq!(old.uuid, new.uuid);
            assert_ne!(old.updated_at, new.updated_at);
            assert_eq!(old.name, new.name);
            assert_ne!(old.price, new.price);
        });
        Ok(())
    }

    #[sqlx::test(fixtures("../../fixtures/items.sql"))]
    async fn test_edit_all(conn: PgPool) -> anyhow::Result<()> {
        let originals = get_valid_uuids(&conn).await?;
        // form item edit
        let args = originals
            .iter()
            .zip(new_names())
            .zip(new_prices())
            .map(|((old, name), price)| ItemEdit {
                targets: old.uuid.to_string(),
                name: Some(name),
                price: Some(price),
            })
            .collect();
        let got = item_edit(&conn, args).await?;
        // assertions
        originals.iter().zip(got).for_each(|(old, new)| {
            assert_eq!(old.uuid, new.uuid);
            assert_ne!(old.updated_at, new.updated_at);
            assert_ne!(old.name, new.name);
            assert_ne!(old.price, new.price);
        });
        Ok(())
    }

    #[sqlx::test]
    /// Ensures that the valdition function checks for uuids.
    /// Db should not be needed, since the uuid check happens before any
    /// connections to the database ar established.
    fn test_uuid_validation(conn: PgPool) {
        let invalid_uuids = new_names();
        let want = ClientError::Uuid(invalid_uuids.join(", "));
        let args = invalid_uuids
            .into_iter()
            .zip(new_prices())
            .map(|(targets, price)| ItemEdit {
                targets,
                name: None,
                price: Some(price),
            })
            .collect();
        let got = validate_edit(&conn, args).await.map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }

    #[sqlx::test(fixtures("../../fixtures/items.sql"))]
    /// Ensure that new name values are properly validated against the
    /// current db values.
    async fn test_unique_constraint(conn: PgPool) -> anyhow::Result<()> {
        let invalid_new_names =
            Arc::<[String]>::from(["Lemon".to_string(), "Mango".into(), "Orange".into()]);
        let want = ClientError::UniqueConstraint("items".into(), invalid_new_names.join(", "));
        let args = get_valid_uuids(&conn)
            .await?
            .iter()
            .zip(invalid_new_names.to_vec())
            .map(|(item, name)| ItemEdit {
                targets: item.uuid.to_string(),
                name: Some(name),
                price: None,
            })
            .collect();
        let got = validate_edit(&conn, args).await.map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }
}
