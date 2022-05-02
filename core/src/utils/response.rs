use rocket::http::{ContentType, Status};
use rocket::{Request, response, Response};
use rocket::response::Responder;
use serde_json::Value;

#[derive(Debug)]
pub struct ApiResponse{
    status: Status,
    payload: Value
}
impl ApiResponse{
    pub fn new(status: Status, payload: Value) -> Self{
        ApiResponse{ status, payload}
    }

    pub fn not_found(error_message: Value) -> Self {
        ApiResponse{ status: Status::NotFound, payload: error_message }
    }
}
impl<'r> Responder<'r, 'static> for  ApiResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        Response::build_from(self.payload.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}