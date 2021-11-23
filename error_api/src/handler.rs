use health::get_health_state;

#[get("/health")]
pub(crate) async fn get_health(_conn: AnalyticsDB) -> Json<health::Health> {
    Json(get_health_state())
}