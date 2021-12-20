/// As you can see, in this file are the non admin Rest-Endpoints.
/// These Endpoints are used to collect/recieve data from the clients using the Opensight SDK's.
use crate::analytics::{AnalyticData, AnalyticEntry, SessionUpdate};
use crate::analytics_dao::AnalyticsDao;
use crate::application::Application;
use crate::db::AnalyticsDB;
use crate::health;
use crate::response::ApiResponse;
use crate::settings::Settings;
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json::json;

use crate::dao::Dao;

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
    let apps = Application::get_all(&settings).await.unwrap();
    for x in apps.iter() {
        if x.application_id == application_id {
            found = true;
        }
    }
    if !found {
        return ApiResponse::new(Status::NotFound, json!({"":""}));
    }
    let analytic_entry = AnalyticEntry::new(analytics.into_inner(), application_id);
    let _result = conn.run(|c| analytic_entry.insert_entry(c)).await;
    ApiResponse::new(Status::Ok, json!({"":""}))
}

#[patch("/<application_id>/session", data = "<session_update>")]
pub(crate) async fn update_session(
    conn: AnalyticsDB,
    application_id: String,
    settings: &State<Settings>,
    session_update: Json<SessionUpdate>,
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
    let dao = AnalyticsDao::new();
    let _result = conn
        .run(move |c| {
            dao.update_entry(
                session_update.session_id.as_str().clone(),
                session_update.session_length.clone(),
                c,
            )
        })
        .await;
    Status::Accepted
}
