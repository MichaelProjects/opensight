extern crate bcrypt;

use std::error::Error;

use crate::{models::user::{User, LoginData}, db::DatabaseConnection};
use bcrypt::{verify};



pub async fn authenticate_user(user: LoginData, conn: DatabaseConnection) -> Result<Option<User>, Box<dyn Error>> {
    let result_user = User::get_user(user.email, conn).await?;
    let to_hash_pw = format!("{}{}", user.password, result_user.pepper);
    let is_vaild = verify(to_hash_pw, result_user.password.as_str())?;
    let option_user: Option<User> = Some(result_user);
    Ok(option_user)
}