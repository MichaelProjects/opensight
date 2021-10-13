use crate::application;
use crate::db::AnalyticsDB;
use crate::application::{Application, ApplicationData, ApplicationType};
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::analytics::{get_all_entries, AnalyticEntry};

#[post("/application", data="<data>")]
pub(crate) async fn insert_application (conn: AnalyticsDB, data: Json<ApplicationData<'_>>) -> Status{
    let application = Application::new(data.application_name, ApplicationType::from_str(data.os));
    let _result = conn.run(|c| application.insert_entry(c)).await;
    Status::Accepted
}

#[get("/application")]
pub(crate) async fn get_applications(conn: AnalyticsDB) -> Json<Vec<Application>>{
    let data = conn.run(|c| application::get_all(c)).await;
    Json(data)
}

#[get("/application/entry")]
pub(crate) async fn get_application_entrys(conn: AnalyticsDB) -> Json<Vec<AnalyticEntry>>{
    let entry = conn.run(|c| get_all_entries(c)).await;
    Json(entry)
}