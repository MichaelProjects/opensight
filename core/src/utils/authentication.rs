use crate::{models::user::{User, LoginData}, db::DatabaseConnection};



pub async fn authenticate_user(user: LoginData, conn: DatabaseConnection) -> Option<String> {
    let result_user = match User::get_user(user.email, conn).await{
        Ok(result_user) => result_user,
        Err(err) => return None
    };
    let to_hash_pw = format!("{}{}", user.password, result_user.pepper);
}