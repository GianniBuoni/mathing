use sqlx::{PgPool, Postgres};

use crate::get_duplicates::get_duplicates;

use super::{user_row::UserPgRow, *};

impl MathingUserService {
    pub(super) async fn handle_get(
        &self,
        req: Request<UserGetRequest>,
    ) -> Result<Response<UserGetResponse>, Status> {
        let req = req.into_inner();
        info!("{:?}", req);

        let conn = DBconn::try_get().await?;
        let names = req.names;

        let users = tokio::time::timeout(DBconn::context(), user_get(conn, names.into()))
            .await
            .map_err(|_| DbError::ContextError)??
            .into_iter()
            .collect();

        Ok(Response::new(UserGetResponse { users }))
    }
}
/// Calls the database to get any matching user entries.
/// Implicity validates and returns any empty or non-unique inputs,
/// and returns any other inputs that don't exist withing an error.
pub(super) async fn user_get(
    conn: &PgPool,
    names: Arc<[String]>,
) -> Result<Vec<UserPgRow>, DbError> {
    // validate names
    if names.is_empty() {
        return Err(DbError::EmptyArgs);
    }
    if let Some(found) = get_duplicates(names.clone()) {
        return Err(DbError::UniqueConstraint("users", found));
    }
    // sql statement
    let mut q = sqlx::QueryBuilder::<Postgres>::new("SELECT * FROM users WHERE name IN ");
    q.push_tuples(names.iter().take(BIND_LIMIT), |mut b, name| {
        b.push_bind(name);
    });
    let rows = q.build_query_as::<UserPgRow>().fetch_all(conn).await?;
    // validate result
    if names.len() != rows.len() {
        let not_found = names
            .iter()
            .filter(|&f| !rows.iter().any(|row| &row.name == f))
            .cloned()
            .collect::<Vec<String>>()
            .join(", ");

        return Err(DbError::EntryNotFound("users", not_found));
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use crate::errors::expected_error;

    use super::*;

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    /// Basic get test; expects query to succeed and return the name inputed at the beginning of the test
    async fn test_user_get(conn: PgPool) -> anyhow::Result<()> {
        let names = vec!["jon".into()];
        let want = names.clone();

        let got = user_get(&conn, names.into())
            .await?
            .into_iter()
            .map(|f| f.name)
            .collect::<Vec<String>>();

        assert_eq!(want, got);
        Ok(())
    }

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    /// Repeated inputs will only return what is found by the database.
    /// This can interfere with the result validation later in the function
    /// that relies on length comparisons.
    /// For that reason get inputs should be unique as well.
    async fn test_repeat_get(conn: PgPool) -> anyhow::Result<()> {
        let want = DbError::UniqueConstraint("users", "jon".into());
        let names = vec!["jon".to_string(); 3].into();
        let got = user_get(&conn, names).await.map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    /// Basic error test; expects query to fail and return the correct DB error
    async fn test_user_get_error(conn: PgPool) -> anyhow::Result<()> {
        let name = "ringo";
        let want = DbError::EntryNotFound("users", name.into());

        let names = vec!["jon".into(), "noodle".into(), name.into()];
        let got = user_get(&conn, names.into()).await.map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }

    #[sqlx::test()]
    /// Sending emty arguments to the query should fail and return DbError::EmptyArg.
    async fn test_empty_error(conn: PgPool) -> anyhow::Result<()> {
        let want = DbError::EmptyArgs;
        let got = user_get(&conn, Arc::new([])).await.map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }
}
