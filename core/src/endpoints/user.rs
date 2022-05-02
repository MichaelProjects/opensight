use rocket::{self, http::Status, State};
use serde_json::{json};
use crate::{models::{health::{Health}, application::ApplicationData, user::LoginData}, utils::settings::Settings};
use rocket::serde::json::Json;
use crate::db::DatabaseConnection;
use crate::models::user::{UserData, User};
use crate::utils::response::ApiResponse;
use crate::utils::authentication;
use crate::utils::jwt::{create_token};


#[post("/user/login", data="<user_data>")]
pub(crate) async fn login(conn: DatabaseConnection, settings: &State<Settings>, user_data: Json<LoginData>) -> ApiResponse {

    let is_correct = match authentication::authenticate_user(user_data.0, conn).await{
        Ok(result) => result,
        Err(_) => return ApiResponse::new(Status::Unauthorized, json!({"error": "login failed"}))
    };
    if is_correct.is_some() {
        match create_token(is_correct.unwrap().id, String::from("area:all"), settings.user_auth.jwt_secret.clone()){
            Ok(token) => return ApiResponse::new(Status::Ok, json!({"token": token})),
            Err(_) => ""
        };
    }
    ApiResponse::new(Status::Unauthorized, json!({"error": "login failed"}))
}

#[post("/user/create", data="<user_data>")]
pub(crate) async fn user_create(conn: DatabaseConnection, user_data: Json<UserData>) -> ApiResponse {
    let new_user = match User::new(&user_data){
        Ok(user) => user,
        Err(_) => return ApiResponse::new(Status::Unauthorized, json!({"error": "Failed to create new user"}))
    };
    match User::insert_user(new_user, conn).await{
        Ok(_) => return ApiResponse::new(Status::Ok, json!({"status": "success"})),
        Err(_) => return ApiResponse::new(Status::Unauthorized, json!({"error": "Failed to create new user"}))
    };
}

#[post("/validate", data="<login_data>")]
pub(crate) async fn validate_user(conn: DatabaseConnection, login_data: Json<LoginData>){
}  

#[get("/refresh?<refresh_token>&<token>&<client_id>")]
pub(crate) async fn refresh_jwt_token(conn: DatabaseConnection, refresh_token: String, token: String, client_id: String){

}
