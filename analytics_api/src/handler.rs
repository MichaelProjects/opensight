use crate::analytics::{AnalyticEntry, AnalyticData};
use rocket::serde::json::Json;
use crate::health;
use crate::db::{Response, AnalyticsDB};

#[get("/health")]
pub(crate) async fn get_health(conn: AnalyticsDB) -> Json<health::Health>{
    Json(health::get_health_state())
}

#[post("/<application_id>/entry", data="<analytics>")]
pub(crate) async fn insert_entry(conn: AnalyticsDB, application_id: String, analytics: Json<AnalyticData>) -> Json<Response>{
    let analytic_entry = AnalyticEntry::new(application_id, analytics.creation_date, analytics.os.clone(), analytics.device_size.clone(), analytics.session_id.clone(), analytics.session_length);
    let result = conn.run(|c| analytic_entry.insert_entry(c)).await;
    info!("{}", result);
    Json(Response::new("a",  false))
}