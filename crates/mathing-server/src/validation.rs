use std::collections::HashSet;

use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::prelude::*;

/// Validates any stored args before passing values to the databse.
/// Check to make sure:
/// - Arguments is not empty
/// - All arguments are unique
/// - (optional) All arguments are not already in the database
/// - (optional) All arguments should already be in the database
#[derive(Default, Debug)]
pub struct Validation {
    args: Arc<[String]>,
    table: String,
    column: String,
    args_exist: bool,
    unique_constraint: bool,
}

impl Validation {
    pub fn new(args: Arc<[String]>, table: &str, column: &str) -> Self {
        Self {
            args,
            table: table.into(),
            column: column.into(),
            ..Default::default()
        }
    }
    /// Ensures that all argements do not already exist in the database
    /// or returns a ClientError::UniqueConstraint.
    /// Mutually exclusive with the args_exist() method.
    pub fn unique_constraint(mut self) -> Self {
        self.unique_constraint = true;
        self
    }
    /// Ensures that all argements already exist in the database
    /// or returns a ClientError::EntryNotFound.
    /// Mutually exclusive with the unique)_constraint() method.
    pub fn args_exist(mut self) -> Self {
        self.args_exist = true;
        self
    }
    fn empty(&self) -> Result<(), ClientError> {
        if self.args.is_empty() {
            return Err(ClientError::EmptyArgs);
        }
        Ok(())
    }
    fn repeated(&self) -> Result<(), ClientError> {
        let mut unique = HashSet::<&str>::new();
        let mut repeat = HashSet::<&str>::new();

        self.args.iter().for_each(|f| {
            if !unique.insert(f) {
                repeat.insert(f);
            }
        });
        if !repeat.is_empty() {
            let repeat = repeat.into_iter().collect::<Vec<&str>>().join(", ");
            return Err(ClientError::RpeatedValue(repeat));
        }

        Ok(())
    }
    async fn select(&self, conn: &PgPool) -> Result<Vec<String>, sqlx::Error> {
        let mut q = QueryBuilder::<Postgres>::new("SELECT ");
        q.push(self.column.as_str());
        q.push(" FROM ");
        q.push(self.table.as_str());
        q.push(" WHERE ");
        q.push(self.column.as_str());
        q.push(" IN");
        q.push_tuples(self.args.iter().take(BIND_LIMIT), |mut b, name| {
            b.push_bind(name);
        });

        q.build_query_scalar::<String>().fetch_all(conn).await
    }
    async fn unique(&self, conn: &PgPool) -> Result<(), ClientError> {
        let found = self.select(conn).await.unwrap_or_default();

        if !found.is_empty() {
            let found = found.join(", ");
            return Err(ClientError::UniqueConstraint(self.table.to_owned(), found));
        }
        Ok(())
    }
    async fn exists(&self, conn: &PgPool) -> Result<(), ClientError> {
        let found = self.select(conn).await.unwrap_or_default();

        if found.len() != self.args.len() {
            let not_found = self
                .args
                .iter()
                .filter(|f| !found.iter().any(|row| &row == f))
                .cloned()
                .collect::<Vec<String>>()
                .join(", ");
            return Err(ClientError::EntryNotFound(self.table.to_owned(), not_found));
        }
        Ok(())
    }
    pub async fn validate(&self, conn: &PgPool) -> Result<(), ClientError> {
        self.empty()?;
        self.repeated()?;
        if self.args_exist {
            self.exists(conn).await?;
        }
        if self.unique_constraint {
            self.unique(conn).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::errors::expected_error;

    use super::*;
    /// passed in args shold not be empty
    #[sqlx::test]
    async fn test_empty(conn: PgPool) {
        let want = ClientError::EmptyArgs;
        let args = vec![].into();
        let val = Validation::new(args, "users", "name");
        let got = val.validate(&conn).await.map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }

    #[sqlx::test]
    /// passed in args should not be repeated
    async fn test_repeated(conn: PgPool) {
        // Because HashSets do not take insert order into account,
        // Every possible permutation of repeated names should be
        // a valid error value.
        let want = ClientError::RpeatedValue("jon, george".into());
        let want_other = ClientError::RpeatedValue("george, jon".into());

        let mut args: Vec<String> = vec!["jon".into(); 3];
        let mut george: Vec<String> = vec!["george".into(); 2];
        args.append(&mut george);

        let got = Validation::new(args.into(), "users", "name")
            .validate(&conn)
            .await
            .map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert!(
                want.to_string() == e.to_string() || want_other.to_string() == e.to_string()
            ),
        }
    }

    #[sqlx::test(fixtures("../fixtures/users.sql"))]
    /// (optional) passed in args should not already be in the database
    async fn test_unique_contraint(conn: PgPool) {
        let want = ClientError::UniqueConstraint("users".into(), "jon, noodle".into());
        let args = vec!["jon".into(), "noodle".into()].into();
        let got = Validation::new(args, "users", "name")
            .unique_constraint()
            .validate(&conn)
            .await
            .map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }

    #[sqlx::test(fixtures("../fixtures/users.sql"))]
    async fn test_not_found(conn: PgPool) {
        let want = ClientError::EntryNotFound("users".into(), "paul, ringo".into());
        let args = vec!["jon".into(), "paul".into(), "ringo".into()].into();
        let got = Validation::new(args, "users", "name")
            .args_exist()
            .validate(&conn)
            .await
            .map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }

    #[sqlx::test(fixtures("../fixtures/users.sql"))]
    /// valid args should not return any errors
    async fn test_validate(conn: PgPool) -> anyhow::Result<()> {
        let args = vec!["ringo".into(), "george".into()].into();
        let table = "users";
        let column = "name";
        let val = Validation::new(args, table, column).unique_constraint();

        val.validate(&conn).await?;
        Ok(())
    }
}
