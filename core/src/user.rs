use chrono::{NaiveDateTime, Utc};
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginData {
    pub loginname: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserData{
    username: String,
    email: String,
    password: String
}


#[derive(Serialize)]
pub struct User{
    user_id: String,
    group_id: String,
    username: String,
    email: String,
    password: String,
    creation_time: NaiveDateTime,
}
impl User{
    pub fn new(data: &Json<UserData>) -> User{
        User{
            user_id: Uuid::new_v4().to_string(),
            group_id: "".to_string(),
            username: data.username.clone(),
            email: data.email.clone(),
            password: data.password.clone(),
            creation_time: Utc::now().naive_utc(),
        }
    }
}