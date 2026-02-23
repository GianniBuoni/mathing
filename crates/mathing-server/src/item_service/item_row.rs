use chrono::{DateTime, Local};
use sqlx::{
    prelude::FromRow,
    types::{Decimal, Uuid},
};

use crate::{errors::ServerError, prelude::mathing_proto::ItemRow};

#[derive(Debug, Default, FromRow)]
pub struct ItemPgRow {
    pub uuid: Uuid,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub name: String,
    pub price: Decimal,
}

impl TryFrom<ItemPgRow> for ItemRow {
    type Error = ServerError;

    fn try_from(value: ItemPgRow) -> Result<ItemRow, Self::Error> {
        let price = f32::try_from(value.price)
            .map_err(|_| ServerError::ConversionError("Decimal", "f32", value.price.to_string()))?;

        Ok(Self {
            uuid: value.uuid.into(),
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
            name: value.name,
            price,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Add;

    use chrono::Duration;

    use super::*;

    /// Basic test to ensure fields are properly remapped
    #[test]
    fn test_into_item_row() -> anyhow::Result<()> {
        let created_at = Local::now();
        let updated_at = Local::now().add(Duration::days(1));
        let name = String::from("noodles");
        let price = 1.99_f32;

        let want = ItemRow {
            uuid: Uuid::nil().to_string(),
            created_at: created_at.to_string(),
            updated_at: updated_at.to_string(),
            name: name.clone(),
            price,
        };

        let got = ItemRow::try_from(ItemPgRow {
            uuid: Uuid::nil(),
            created_at,
            updated_at,
            name,
            price: Decimal::from_f32_retain(price).unwrap(),
        })?;

        assert_eq!(want, got);
        Ok(())
    }
}
