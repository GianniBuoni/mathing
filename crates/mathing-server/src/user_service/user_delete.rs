use super::*;

impl MathingUserService {
    pub(super) fn handle_delete(
        &self,
        req: Request<UserDeleteRequest>,
    ) -> Result<Response<UserDeleteResponse>, Status> {
        let req = req.into_inner();

        info!("{:?}", req);
        // establish db connection
        // parse db response
        // return number of rows affected
        let message = UserDeleteResponse { rows_affected: 1 };
        Ok(Response::new(message))
    }
}
