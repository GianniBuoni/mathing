use super::*;

impl MathingUserService {
    pub(super) fn handle_create(
        &self,
        _req: Request<UserCreateRequest>,
        // establish db connection
        // parse db response
        // return new uuid of user
    ) -> Result<Response<UserCreateResponse>, Status> {
        let message = UserCreateResponse { uuid: 1 };
        Ok(Response::new(message))
    }
}
