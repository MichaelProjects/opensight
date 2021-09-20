use crate::analytics::{AnalyticEntry, AnalyticData, get_all_entrys};
use rocket::serde::json::Json;
use crate::{health, application};
use crate::db::{AnalyticsDB};
use crate::application::{Application, ApplicationData, ApplicationType};
use rocket::http::Status;

use crate::application_dao::ApplicationDao;
use crate::dao::Dao;

#[get("/health")]
pub(crate) async fn get_health(_conn: AnalyticsDB) -> Json<health::Health>{
    Json(health::get_health_state())
}

#[post("/<application_id>/entry", data="<analytics>")]
pub(crate) async fn insert_entry(conn: AnalyticsDB, application_id: String, analytics: Json<AnalyticData>) -> Status{
    let mut found = false;
    let apps = conn.run(|c| ApplicationDao::new().get_all(c)).await;
    for x in apps.iter(){
        if x.application_id == application_id{
            found = true;
        }
    }
    if !found{
        return Status::NotFound
    }
    let analytic_entry = AnalyticEntry::new(analytics.into_inner(), application_id);
    let _result = conn.run(|c| analytic_entry.insert_entry(c)).await;
    Status::Accepted
}

#[post("/admin/application", data="<data>")]
pub(crate) async fn insert_application (conn: AnalyticsDB, data: Json<ApplicationData<'_>>) -> Status{
    let application = Application::new(data.application_name, ApplicationType::from_str(data.os));
    let _result = conn.run(|c| application.insert_entry(c)).await;
    Status::Accepted
}

#[get("/admin/application")]
pub(crate) async fn get_applications(conn: AnalyticsDB) -> Json<Vec<Application>>{
    let data = conn.run(|c| application::get_all(c)).await;
    Json(data)
}

#[get("/admin/application/entry")]
pub(crate) async fn get_application_entrys(conn: AnalyticsDB) -> Json<Vec<AnalyticEntry>>{
    let entry = conn.run(|c| get_all_entrys(c)).await;
    Json(entry)
}