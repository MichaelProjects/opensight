use diesel::{QueryResult, PgConnection};
use super::schema::opensight_user;
use crate::user::User;
use diesel::prelude::*;


pub fn insert_user(user: User, conn: &PgConnection) -> Result<User, diesel::result::Error> {
    let response: QueryResult<User> = diesel::insert_into(opensight_user::table)
    .values(user)
    .get_result(conn);
    response
}

pub fn get_user(username: String, conn: &PgConnection) -> Result<User, diesel::result::Error>{
    let response: QueryResult<User> = opensight_user::table.filter(opensight_user::username.eq(username)).first(conn);
    response
}