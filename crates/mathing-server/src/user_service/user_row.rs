use std::sync::Arc;

use chrono::{DateTime, Local};
use sqlx::types::uuid;

use crate::prelude::mathing_proto::UserRow;

/// Raw PG row that needs to be converted into string data
/// to mass to an RPC message.
#[derive(Debug)]
pub struct UserPgRow {
    pub uuid: uuid::Uuid,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub name: Arc<str>,
}

impl From<UserPgRow> for UserRow {
    fn from(value: UserPgRow) -> Self {
        Self {
            uuid: value.uuid.into(),
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
            name: value.name.to_string(),
        }
    }
}
