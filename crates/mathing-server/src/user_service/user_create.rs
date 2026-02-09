use super::*;

impl MathingUserService {
    pub(super) fn handle_create(
        &self,
        req: Request<UserCreateRequest>,
    ) -> Result<Response<UserCreateResponse>, Status> {
        let req = req.into_inner();

        info!("{:?}", req);
        // establish db connection
        // parse db response
        // return new uuid of user
        let message = UserCreateResponse { uuid: 1 };
        Ok(Response::new(message))
    }
}
