use rocket;
use crate::{health::{Health}, application::ApplicationData};
use rocket::serde::json::Json;
use crate::db::DatabaseConnection;
use crate::health;
use crate::user::{UserData, User};
use crate::application::Application;

#[get("/health")]
pub(crate) fn get_health(conn: DatabaseConnection,) -> Json<Health> {
    Json(Health::new(200, true))

}

#[post("/application", data="<app_data>")]
pub(crate) async fn create_application(conn: DatabaseConnection, app_data: Json<ApplicationData>){
    let app = Application::new(&app_data.name, &app_data.os);
    let response = conn.run(|c| app.insert(c)).await;

}

#[get("/application/<id>")]
pub(crate) async fn get_application(conn: DatabaseConnection, id: String) -> Json<Application>{
    let app = Application::get(id.clone(), conn);
    Json(app)
}

#[post("/login", data="<user_data>")]
pub(crate) async fn login(conn: DatabaseConnection, user_data: Json<UserData>){
    let user = User::new(user_data);

}
