use chrono::{NaiveDateTime, Utc};
use rocket::State;
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use serde_json::json;
use crate::analyse::{analyse_user, DayData};
use crate::analytics;
use crate::application::Application;
use crate::db::AnalyticsDB;
use crate::response::ApiResponse;
use crate::settings::Settings;

#[derive(Debug)]
pub struct ApiKey<'r>(&'r str);

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

async fn vaildate_key(key: &str, app_id: &String){
}

#[get("/<application_id>/analyse?<start>&<end>")]
pub(crate) async fn get_analyse_data(
    conn: AnalyticsDB,
    application_id: String, 
    key: ApiKey<'_>, 
    start: Option<i64>, 
    end: Option<i64>) -> ApiResponse{
    println!("{:?}", &key);
    vaildate_key(key.0, &application_id).await;
    let start = NaiveDateTime::from_timestamp(start.unwrap(), 0);
    let end = NaiveDateTime::from_timestamp(end.unwrap(), 0);
    let entry_data = match analytics::get_timeframe_entries(application_id, conn, start, end).await{
        Ok(entries) => entries,
        Err(_) => {
            return ApiResponse::new(Status::BadRequest, json!({
                "error": "Could not get entries"
            }));
        }
    };
    let processed_data: Vec<DayData> = analyse_user(entry_data);
    ApiResponse::new(Status::Ok, json!({"data": processed_data}))
}


#[get("/<application_id>/analyse/user?<start>&<end>")]
pub(crate) async fn get_analyse_user(
    conn: AnalyticsDB,
    application_id: String, 
    key: ApiKey<'_>, 
    start: Option<i64>, 
    end: Option<i64>) -> ApiResponse{
    println!("{:?}", &key);
    vaildate_key(key.0, &application_id).await;
    let start = NaiveDateTime::from_timestamp(start.unwrap(), 0);
    let end = NaiveDateTime::from_timestamp(end.unwrap(), 0);
    let entry_data = match analytics::get_newuser_in_timeframe(application_id, conn, start, end).await{
        Ok(entries) => entries,
        Err(_) => {
            return ApiResponse::new(Status::BadRequest, json!({
                "error": "Could not get entries"
            }));
        }
    };
    
    ApiResponse::new(Status::Ok, json!({"data": entry_data}))
}