extern crate diesel;

use std::error::Error;

use crate::daos::user_dao::{insert_user, get_user};
use crate::{db::DatabaseConnection};

use crate::schema::users;
use chrono::{NaiveDateTime, Utc};
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


#[derive(Serialize, Deserialize, Debug)]
pub struct LoginData {
    pub email: String,
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
    id: String,
    group_id: String,
    username: String,
    email: String,
    pub password: String,
    pub pepper: String,
    creation_time: NaiveDateTime,
}
impl User{
    pub fn new(data: &Json<UserData>) -> User {
        let pepper = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect();
        User{
            id: Uuid::new_v4().to_string(),
            group_id: "".to_string(),
            username: data.username.clone(),
            email: data.email.clone(),
            password: data.password.clone(),
            pepper,
            creation_time: Utc::now().naive_utc(),
        }
    }
    pub async fn insert_user(user: User, conn: DatabaseConnection) -> Result<User, Box<dyn Error>> {
        let user = conn.run(|c| insert_user(user, c)).await?;
        Ok(user)
    }
    pub async fn get_user(email: String, conn: DatabaseConnection) -> Result<User, Box<dyn Error>> {
        let user = conn.run(|c| get_user(email, c)).await?;
        Ok(user)
    }
}



