use sqlx::PgPool;

use super::{user_row::UserPgRow, *};

async fn user_edit(conn: &PgPool, old: &str, new: &str) -> Result<UserPgRow, DbError> {
    let mut tx = conn.begin().await?;
    // Check if old user name exists
    let uuid = super::user_get::user_get(conn, old).await?.uuid;
    // Check if new name is already taken
    if let Ok(_) = super::user_get::user_get(conn, new).await {
        return Err(DbError::UniqueConstraint("users", "name"));
    }

    Ok(sqlx::query_as!(
        UserPgRow,
        "UPDATE users SET name=$1, updated_at=CURRENT_TIMESTAMP WHERE uuid=$2 RETURNING *",
        new,
        uuid
    )
    .fetch_one(&mut *tx)
    .await?)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    /// Basic test for editing a user name and if
    /// updated field is actually updated.
    async fn test_user_edit(conn: PgPool) -> anyhow::Result<()> {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let (old, new) = ("jon", "paul");
        let got = user_edit(&conn, old, new).await?;

        assert_eq!(new, got.name.as_ref());
        assert_ne!(got.created_at, got.updated_at);
        Ok(())
    }
    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    /// Test to ensure unique constraints of the table
    /// are upheld by the edit funcition.
    /// A client can successfully pass a name already in the database,
    /// which the server should respond with an error.
    async fn test_user_edit_invalid_arg(conn: PgPool) -> anyhow::Result<()> {
        let want = DbError::UniqueConstraint("users", "name");
        let (old, new) = ("jon", "noodle");
        let got = user_edit(&conn, old, new).await.map(|u| {
            let message = format!("Test expected an error but returned: {u:?}");
            anyhow::Error::msg(message)
        });

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }
    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    /// Test that expects a entry not found error.
    /// The database should check if the given name exists before
    /// attempting to edit it.
    async fn test_user_edit_missing_arg(conn: PgPool) -> anyhow::Result<()> {
        let want = DbError::EntryNotFound("users", "paul".into());
        // These args will both be invalid, but
        // the non-existent user should be detected frist.
        let (old, new) = ("paul", "noodle");
        let got = user_edit(&conn, old, new).await.map(|u| {
            let message = format!("Test expected an error but returned: {u:?}");
            anyhow::Error::msg(message)
        });

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        };
        Ok(())
    }
}
