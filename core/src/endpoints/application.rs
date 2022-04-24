use rocket::{self, http::Status};
use serde_json::{json};
use crate::{models::{health::{Health}, application::ApplicationData, user::LoginData}, utils::oauth::{ApiKey, validate_token}};
use rocket::serde::json::Json;
use crate::db::DatabaseConnection;
use crate::models::user::{UserData, User};
use crate::models::application::Application;
use crate::utils::response::ApiResponse;


#[post("/application", data="<app_data>")]
pub(crate) async fn create_application(conn: DatabaseConnection, app_data: Json<ApplicationData>, key: ApiKey<'_>) -> ApiResponse{
    validate_token(key.0).await;
    let apps = Application::get_all(&conn).await.unwrap();
    for app in apps.iter(){
        if app.application_name == app_data.name {
            return ApiResponse::new(Status::Conflict, json!({"message": "App does already exist"}));
        }
    }
    let app = Application::new(&app_data.name, &app_data.os, &app_data.package_name);
    
    match Application::insert(app.clone(), conn).await{
        Ok(_) => ApiResponse::new(Status::Created, json!({"data": app, "message": "App created"})),
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
