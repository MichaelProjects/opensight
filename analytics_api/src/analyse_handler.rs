use chrono::{NaiveDateTime, Utc};
use rocket::State;
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use serde_json::json;
use crate::analyse::{sort_data_to_day, display_sizes, calc_average_session_length, sort_user_to_day, version_analysis, DayData};
use crate::{analytics, session_dao};
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
    ApiResponse::new(Status::Ok, json!({"data": entry_data}))
}

#[get("/<application_id>/analyse/user?<start>&<end>")]
pub(crate) async fn get_analyse_user(
    conn: AnalyticsDB,
    application_id: String, 
    key: ApiKey<'_>, 
    start: Option<i64>, 
    end: Option<i64>) -> ApiResponse{
    vaildate_key(key.0, &application_id).await;
    let start = NaiveDateTime::from_timestamp(start.unwrap(), 0);
    let end = NaiveDateTime::from_timestamp(end.unwrap(), 0);
    println!("{:?}", &end);
    let entry_data = match session_dao::get_session_count_in_timeframe(application_id, start, end, conn).await{
        Ok(entries) => entries,
        Err(_) => {
            return ApiResponse::new(Status::BadRequest, json!({
                "error": "Could not get entries"
            }));
        }
    };
    let processed_data: Vec<DayData> = sort_user_to_day(entry_data).await;
    ApiResponse::new(Status::Ok, json!({"data": processed_data}))
    
}

#[get("/<application_id>/analyse/user/session?<start>&<end>")]
pub(crate) async fn get_analyse_session_length(
    conn: AnalyticsDB,
    application_id: String, 
    key: ApiKey<'_>, 
    start: Option<i64>, 
    end: Option<i64>) -> ApiResponse{
    println!("{:?}", &key);
    vaildate_key(key.0, &application_id).await;
    let start = NaiveDateTime::from_timestamp(start.unwrap(), 0);
    let end = NaiveDateTime::from_timestamp(end.unwrap(), 0);
    let entry_data = match session_dao::get_sessions_in_timeframe(application_id, start, end, conn).await{
        Ok(entries) => entries,
        Err(_) => {
            return ApiResponse::new(Status::BadRequest, json!({
                "error": "Could not get entries"
            }));
        }
    };
    let processed_data: Vec<DayData> = calc_average_session_length(entry_data).await;
    ApiResponse::new(Status::Ok, json!({"data": processed_data}))
    
}

#[get("/<application_id>/analyse/user/new?<start>&<end>")]
pub(crate) async fn get_analyse_new_user(
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
    let processed_data: Vec<DayData> = sort_data_to_day(entry_data).await;
    ApiResponse::new(Status::Ok, json!({"data": processed_data}))
    
}

#[get("/<application_id>/analyse/app/version?<start>&<end>")]
pub(crate) async fn get_version_info(
    conn: AnalyticsDB,
    application_id: String, 
    key: ApiKey<'_>, 
    start: Option<i64>, 
    end: Option<i64>) -> ApiResponse{
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
    let processed_data: Vec<DayData> = version_analysis(entry_data).await;
    ApiResponse::new(Status::Ok, json!({"data": processed_data}))    
}

#[get("/<application_id>/analyse/device/display?<start>&<end>")]
pub(crate) async fn get_device_display(
    conn: AnalyticsDB,
    application_id: String, 
    key: ApiKey<'_>, 
    start: Option<i64>, 
    end: Option<i64>) -> ApiResponse{
    vaildate_key(key.0, &application_id).await;
    let start = NaiveDateTime::from_timestamp(start.unwrap(), 0);
    let end = NaiveDateTime::from_timestamp(end.unwrap(), 0);
    println!("{:?}", &start);
    let entry_data = match analytics::get_timeframe_entries(application_id, conn, start, end).await{
        Ok(entries) => entries,
        Err(_) => {
            return ApiResponse::new(Status::BadRequest, json!({
                "error": "Could not get entries"
            }));
        }
    };
    let processed_data: Vec<i64> = display_sizes(entry_data).await;
    ApiResponse::new(Status::Ok, json!({"data": {"height": processed_data[0], "width": processed_data[1]}}))    
}