use crate::analytics::{AnalyticEntry, AnalyticData};
use rocket::serde::json::Json;
use crate::{health, Storage};
use crate::db::{Response, AnalyticsDB};
use rocket::State;
use crate::application::{Application, ApplicationData, ApplicationType};

#[get("/health")]
pub(crate) async fn get_health(conn: AnalyticsDB) -> Json<health::Health>{
    Json(health::get_health_state())
}

#[post("/<application_id>/entry", data="<analytics>")]
pub(crate) async fn insert_entry(apps: &State<Storage>,conn: AnalyticsDB, application_id: String, analytics: Json<AnalyticData>) -> Json<Response>{
    println!("{:?}", apps.apps);
    let analytic_entry = AnalyticEntry::new(application_id, analytics.creation_date, analytics.os.clone(), analytics.device_size.clone(), analytics.session_id.clone(), analytics.session_length);
    let result = conn.run(|c| analytic_entry.insert_entry(c)).await;
    info!("{}", result);
    Json(Response::new("a",  false))
}

pub(crate) async fn insert_application(conn: AnalyticsDB, data: Json<ApplicationData<'_>>) -> Json<Response>{
    let application = Application::new(data.application_name, ApplicationType::from_str(data.os));
    let result = conn.run(|c| application.insert_entry(c)).await;
    info!("{}", result);

    Json(Response::new("a",  false))
}