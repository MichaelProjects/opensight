use rocket;
use crate::health::{Health};
use rocket::serde::json::Json;
use crate::db::DatabaseConnection;

#[get("/health")]
pub(crate) fn get_health(conn: DatabaseConnection,) -> Json<Health> {
    Json(Health::new(200, true))
}

#[post("/application")]
pub(crate) async fn create_application(){
    
}

#[get("/application/<id>")]
pub(crate) async fn get_application(id: u32){
    
}

#[post("/login")]
pub(crate) async fn login(){
    
}
