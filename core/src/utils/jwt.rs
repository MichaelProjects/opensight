use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct ApiKey<'r>(pub &'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        match req.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) => Outcome::Success(ApiKey(key)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    company: String,
    user_id: String,
    scope: String,
    exp: usize
}

pub fn create_token(){
    
}