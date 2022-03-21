/// As you can see, in this file are the non admin Rest-Endpoints.
/// These Endpoints are used to collect/recieve data from the clients using the Opensight SDK's.
use crate::analytics::{self, AnalyticData, AnalyticEntry};
use crate::daos::analytics_dao::AnalyticsDao;
use crate::application::Application;
use crate::dao::Dao;
use crate::db::AnalyticsDB;
use crate::health;
use crate::response::ApiResponse;
use crate::daos::session_dao::{self, SessionIn};
use crate::settings::Settings;
use chrono::NaiveDateTime;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::json;

#[get("/health")]
pub(crate) async fn get_health(_conn: AnalyticsDB) -> Json<health::Health> {
    Json(health::get_health_state())
}

#[post("/<application_id>/session", data = "<analytics>")]
pub(crate) async fn insert_entry(
    conn: AnalyticsDB,
    application_id: String,
    settings: &State<Settings>,
    analytics: Json<AnalyticData>,
) -> ApiResponse {
    let mut found = false;
    let apps = match Application::get_all(&settings).await {
        Ok(app) => app,
        Err(err) => {
            log::error!("{:?}",err);
            return ApiResponse::new(Status::InternalServerError, json!({}));
        }
    };
    for x in apps.iter() {
        if x.application_id == application_id {
            found = true;
        }
    }
    if !found {
        return ApiResponse::new(Status::NotFound, json!({"error":"Application not found"}));
    }
    let analytic_entry = AnalyticEntry::new(analytics.into_inner(), application_id);
    let session = session_dao::Session::from_analytic_entry(&analytic_entry);
    match analytics::insert_entry(analytic_entry, session, conn).await {
        Ok(response) => ApiResponse::new(Status::Ok, response),
        Err(err) => {
            log::error!("{:?}",err);
            ApiResponse::new( Status::InternalServerError, json!({"error":"An error occured"}))},
    }
}

#[patch("/<application_id>/session", data = "<new_data>")]
pub(crate) async fn update_session(
    conn: AnalyticsDB,
    application_id: String,
    settings: &State<Settings>,
    new_data: Json<SessionIn>,
) -> Status {
    let mut found = false;
    let apps: Vec<Application> = Application::get_all(settings).await.unwrap();
    for x in apps.iter() {
        if x.application_id == application_id {
            found = true;
        }
    }
    if !found {
        return Status::NotFound;
    }
    match session_dao::update_session(new_data.session_id.clone() ,new_data.length.clone(), conn, new_data.is_first_today.clone()).await{
        Ok(_) => Status::Ok,
        Err(err) => {
            log::error!("{:?}",err);
            Status::InternalServerError
        }
    }
}

#[get("/<application_id>/session?<limit>&<start>&<end>")]
pub(crate) async fn get_sessions(conn: AnalyticsDB, application_id: String, limit: Option<i64>, start: Option<i64>, end: Option<i64>) -> ApiResponse {
    let start = NaiveDateTime::from_timestamp(start.unwrap() / 1000, 0);
    let end = NaiveDateTime::from_timestamp(end.unwrap() / 1000, 0);
    
    let mut final_limit: i64 = 100;
    if limit.is_some(){
        final_limit = limit.unwrap();
    }
    println!("{:?}", &end);
    let sessions: Vec<AnalyticEntry> = analytics::get_all_entries(application_id, conn, final_limit, start, end).await;
    ApiResponse::new(Status::Ok, json!({ "data": sessions }))
}
