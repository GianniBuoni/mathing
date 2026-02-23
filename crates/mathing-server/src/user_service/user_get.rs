use sqlx::{PgPool, Postgres};

use super::{user_row::UserPgRow, *};

impl MathingUserService {
    pub(super) async fn handle_get(
        &self,
        req: Request<UserGetRequest>,
    ) -> Result<Response<UserGetResponse>, Status> {
        let req = req.into_inner();
        info!("{:?}", req);

        let conn = DBconn::try_get().await?;
        let args = Arc::<[String]>::from(req.names);

        let users = tokio::time::timeout(DBconn::context(), async {
            validate_get(conn, args.clone()).await?;
            Ok::<Vec<UserPgRow>, Status>(user_get(conn, args).await?)
        })
        .await
        .map_err(|_| DbError::ContextError)??
        .into_iter()
        .collect();

        Ok(Response::new(UserGetResponse { users }))
    }
}

async fn validate_get(conn: &PgPool, args: Arc<[String]>) -> Result<(), ClientError> {
    Validation::new(args, "users", "name")
        .args_exist()
        .validate(conn)
        .await
}

/// Calls the database to get any matching user entries.
async fn user_get(conn: &PgPool, names: Arc<[String]>) -> Result<Vec<UserPgRow>, DbError> {
    // sql statement
    let mut q = sqlx::QueryBuilder::<Postgres>::new("SELECT * FROM users WHERE name IN ");
    q.push_tuples(names.iter().take(BIND_LIMIT), |mut b, name| {
        b.push_bind(name);
    });
    let rows = q.build_query_as::<UserPgRow>().fetch_all(conn).await?;

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
    /// Basic error test; expects query to fail and return the correct DB error
    async fn test_user_get_error(conn: PgPool) {
        let name = "ringo";
        let want = ClientError::EntryNotFound("users".into(), name.into());

        let names = vec!["jon".into(), "noodle".into(), name.into()];
        let got = validate_get(&conn, names.into()).await.map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }

    #[sqlx::test]
    /// Sending emty arguments to the server should fail and return ClientError::EmptyArg.
    async fn test_empty_error(conn: PgPool) {
        let want = ClientError::EmptyArgs;
        let args = vec![].into();
        let got = validate_get(&conn, args).await.map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }
}
