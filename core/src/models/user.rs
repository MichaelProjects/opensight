extern crate diesel;

use std::error::Error;

use crate::daos::user_dao::{insert_user, get_user};
use crate::{db::DatabaseConnection};

use crate::schema::users;
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


#[derive(Serialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User{
    userid: String,
    group_id: String,
    username: String,
    email: String,
    password: String,
    creation_time: NaiveDateTime,
}
impl User{
    pub fn new(data: &Json<UserData>) -> User{
        User{
            userid: Uuid::new_v4().to_string(),
            group_id: "".to_string(),
            username: data.username.clone(),
            email: data.email.clone(),
            password: data.password.clone(),
            creation_time: Utc::now().naive_utc(),
        }
    }
    pub async fn insert_user(user: User, conn: DatabaseConnection) -> Result<User, Box<dyn Error>> {
        let user = conn.run(|c| insert_user(user, c)).await?;
        Ok(user)
    }
    pub async fn get_user(username: String, conn: DatabaseConnection) -> Result<User, Box<dyn Error>> {
        let user = conn.run(|c| get_user(username, c)).await?;
        Ok(user)
    }
}



