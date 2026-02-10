use super::*;

use std::sync::Arc;

use chrono::{DateTime, Local};
use sqlx::types::uuid;

#[derive(Debug)]
pub struct UserRow {
    pub uuid: uuid::Uuid,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub name: Arc<str>,
}

impl From<UserRow> for UserCreateResponse {
    fn from(value: UserRow) -> Self {
        Self {
            uuid: value.uuid.into(),
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
            name: value.name.to_string(),
        }
    }
}
