use rocket_sync_db_pools::database;
use serde::{Deserialize, Serialize};

#[database("postgres_url")]
pub struct AnalyticsDB(diesel::PgConnection);

#[derive(Serialize, Deserialize)]
pub struct Response {
    message: String,
    error: bool,
}
impl Response {
    pub fn new(message: &str, error: bool) -> Self {
        Response {
            message: String::from(message),
            error,
        }
    }
}
