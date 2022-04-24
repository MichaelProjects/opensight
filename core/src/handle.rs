use rocket::{self, http::Status};
use serde_json::{json};
use crate::models::{health::{Health}, application::ApplicationData, user::LoginData};
use rocket::serde::json::Json;
use crate::db::DatabaseConnection;
use crate::models::user::{UserData, User};
use crate::models::application::Application;
use crate::utils::response::ApiResponse;


#[get("/health")]
pub(crate) fn get_health(_conn: DatabaseConnection,) -> Json<Health> {
    Json(Health::new(200, true))

}

