use chrono::{DateTime, Local};
use sqlx::{prelude::FromRow, types::uuid};

use crate::prelude::mathing_proto::UserRow;

/// Raw PG row that needs to be converted into string data
/// to mass to an RPC message.
#[derive(Debug, FromRow)]
pub struct UserPgRow {
    pub uuid: uuid::Uuid,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub name: String,
}

impl From<UserPgRow> for UserRow {
    fn from(value: UserPgRow) -> Self {
        Self {
            uuid: value.uuid.into(),
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
            name: value.name,
        }
    }
}

impl FromIterator<UserPgRow> for Vec<UserRow> {
    fn from_iter<T: IntoIterator<Item = UserPgRow>>(iter: T) -> Self {
        iter.into_iter().map(UserRow::from).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Add;

    use chrono::Duration;

    use super::*;
    /// Basic test to make sure that fields are correctly remapped
    /// to a UserRow struct
    #[test]
    fn test_user_row_into() {
        let created_at = Local::now();
        let updated_at = Local::now().add(Duration::days(1));

        let pg_row = UserPgRow {
            uuid: uuid::Uuid::nil(),
            created_at,
            updated_at,
            name: "noodle".into(),
        };

        let want = UserRow {
            uuid: uuid::Uuid::nil().to_string(),
            created_at: created_at.to_string(),
            updated_at: updated_at.to_string(),
            name: "noodle".into(),
        };
        let want = format!("{:?}", want);
        let got = format!("{:?}", Into::<UserRow>::into(pg_row));

        assert_eq!(want, got);
    }
}
