use std::error::Error;
use std::ops::Add;

use jsonwebtoken::{encode, Header, EncodingKey};
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Duration};
use chrono::prelude::*;


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
    issued: Datetime<Utc>,
    exp: Datetime<Utc>
}

pub fn create_token(user_id: String, scope: String, secret: String) -> Result<String, Box<dyn Error>>{
    let expire_time = Utc::now() + Duration::minutes(60);
    let opensight_claims = Claims {
        company: "opensight".to_owned(),
        user_id: user_id,
        scope: scope,
        issued: Utc::now(),
        exp: expire_time
    };
    let token = encode(&Header::default(), &opensight_claims, &EncodingKey::from_secret(secret.as_ref()))?;
    Ok(token)
}