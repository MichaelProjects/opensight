use crate::analytics::{AnalyticEntry, AnalyticData};
use rocket::serde::json::Json;
use crate::{health};
use crate::db::{Response, AnalyticsDB};
use rocket::State;
use crate::application::{Application, ApplicationData, ApplicationType};
use rocket::http::Status;
use crate::cache::Cache;
use log::{debug};
use crate::application_dao::ApplicationDao;
use crate::dao::Dao;

#[get("/health")]
pub(crate) async fn get_health(conn: AnalyticsDB) -> Json<health::Health>{
    Json(health::get_health_state())
}

#[post("/<application_id>/entry", data="<analytics>")]
pub(crate) async fn insert_entry(apps: &State<Cache>,conn: AnalyticsDB, application_id: String, analytics: Json<AnalyticData>) -> Status{
    let mut found = false;
    let dao = ApplicationDao::new();
    let apps = conn.run(move |c| dao.get_all(c)).await;
    for x in apps.iter(){
        if x.application_id == application_id{
            found = true;
        }
    }
    if found == false{
        return Status::NotFound
    }
    let analytic_entry = AnalyticEntry::new(application_id, analytics.creation_date, analytics.os.clone(), analytics.device_size.clone(), analytics.session_id.clone(), analytics.session_length);
    let result = conn.run(|c| analytic_entry.insert_entry(c)).await;
    Status::Accepted
}

#[post("/admin/application", data="<data>")]
pub(crate) async fn insert_application(cache: &State<Cache>, conn: AnalyticsDB, data: Json<ApplicationData<'_>>) -> Status{
    let application = Application::new(data.application_name, ApplicationType::from_str(data.os));
    let result = conn.run(|c| application.insert_entry(c)).await;
    Status::Accepted
}