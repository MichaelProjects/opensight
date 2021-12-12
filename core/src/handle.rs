use rocket;
use crate::health::{check_health,};
use crate::db::DatabaseConnection;

#[get("/health")]
pub(crate) async fn get_health(conn: DatabaseConnection,) {
    check_health(conn).await;
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
