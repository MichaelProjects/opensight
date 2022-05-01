use crate::{daos::projects_dao::ProjectIn};
use rocket::{self, http::Status};
use serde_json::{json};
use crate::{models::{health::{Health}, application::ApplicationData, user::LoginData}, utils::jwt::{ApiKey}};
use crate::db::DatabaseConnection;
use rocket::serde::json::Json;
use crate::utils::response::ApiResponse;
use crate::models::project::Project;



#[post("/projects", data="<project_data>")]
pub(crate) async fn create_project_endpoint(conn: DatabaseConnection, project_data: Json<ProjectIn>, token: ApiKey<'_>) -> ApiResponse {
    let projects = Project::select_all(&conn).await.unwrap();
    
    for project in projects.iter(){
        if &project.projects_name == &project_data.projects_name {
            return ApiResponse::new(Status::Conflict, json!({"message": "Project already exists"}));
        }
    }

    let project = Project::new(project_data.0);
    match Project::insert(project, &conn).await{
        Ok(_) => ApiResponse::new(Status::Created, json!({"message": "Project created"})),
        Err(_) => ApiResponse::new(Status::BadRequest, json!({"message": "an error occurred"}))
    }
}