use super::*;

impl MathingUserService {
    pub(super) fn handle_delete(
        &self,
        _req: Request<UserDeleteRequest>,
    ) -> Result<Response<UserDeleteResponse>, Status> {
        let message = UserDeleteResponse { rows_affected: 1 };
        Ok(Response::new(message))
    }
}
