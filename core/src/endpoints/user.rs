use rocket::{self, http::Status};
use serde_json::{json};
use crate::models::{health::{Health}, application::ApplicationData, user::LoginData};
use rocket::serde::json::Json;
use crate::db::DatabaseConnection;
use crate::models::user::{UserData, User};
use crate::utils::response::ApiResponse;


#[post("/login", data="<user_data>")]
pub(crate) async fn login(conn: DatabaseConnection, user_data: Json<UserData>){
    let user = User::new(&user_data);

}

#[post("/validate", data="<login_data>")]
pub(crate) async fn validate_user(conn: DatabaseConnection, login_data: Json<LoginData>){
}  

#[get("/refresh?<refresh_token&<token>&<client_id>")]
pub(crate) async fn refresh_jwt_token(conn: DatabaseConnection, refresh_token: String, token: String, client_id: String){

}
