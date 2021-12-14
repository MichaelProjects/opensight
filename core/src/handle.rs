use rocket;
use crate::{health::{Health}, application::ApplicationData};
use rocket::serde::json::Json;
use crate::db::DatabaseConnection;
use crate::application::Application;

#[get("/health")]
pub(crate) fn get_health(conn: DatabaseConnection,) -> Json<Health> {
    Json(Health::new(200, true))
}

#[post("/application", data="<app_data>")]
pub(crate) async fn create_application(conn: DatabaseConnection, app_data: Json<ApplicationData>){
    let app = Application::new(app_data.name.clone(), app_data.os);
}

#[get("/application/<id>")]
pub(crate) async fn get_application(id: u32){
    
}

#[post("/login")]
pub(crate) async fn login(){
    
}
