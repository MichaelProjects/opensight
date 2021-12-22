use rocket::{self, http::Status};
use serde_json::{json};
use crate::{health::{Health}, application::ApplicationData, user::LoginData};
use rocket::serde::json::Json;
use crate::db::DatabaseConnection;
use crate::user::{UserData, User};
use crate::application::Application;
use crate::response::ApiResponse;


#[get("/health")]
pub(crate) fn get_health(_conn: DatabaseConnection,) -> Json<Health> {
    Json(Health::new(200, true))

}

#[post("/application", data="<app_data>")]
pub(crate) async fn create_application(conn: DatabaseConnection, app_data: Json<ApplicationData>) -> ApiResponse{
    let apps = Application::get_all(&conn).await.unwrap();
    for app in apps.iter(){
        if app.application_name == app_data.name {
            return ApiResponse::new(Status::Conflict, json!({"message": "App does already exist"}));
        }
    }
    let app = Application::new(&app_data.name, &app_data.os, &app_data.package_name);
    match Application::insert(app, conn).await{
        Ok(_) => ApiResponse::new(Status::Created, json!({"message": "App created"})),
        Err(_) => ApiResponse::new(Status::InternalServerError, json!({"message": "Error creating app"}))
    }
}

#[get("/application/<id>")]
pub(crate) async fn get_application(conn: DatabaseConnection, id: String) -> ApiResponse{
    match Application::get(id.clone(), conn).await{
        Ok(app) => ApiResponse::new(Status::Ok, json!({"error": false, "data": app})),
        Err(_) => ApiResponse::new(Status::BadRequest, json!({"message": "an error occurred"}))
    }

}
#[get("/application")]
pub(crate) async fn get_all_application(conn: DatabaseConnection) -> ApiResponse{
    match Application::get_all(&conn).await{
        Ok(apps) => ApiResponse::new(Status::Ok, json!({"data": apps})),
        Err(_) => ApiResponse::new(Status::BadRequest, json!({"message": "an error occurred"}))
    }
}

#[post("/login", data="<user_data>")]
pub(crate) async fn login(conn: DatabaseConnection, user_data: Json<UserData>){
    let user = User::new(&user_data);

}

#[post("/validate", data="<login_data>")]
pub(crate) async fn validate_user(conn: DatabaseConnection, login_data: Json<LoginData>){
}  
