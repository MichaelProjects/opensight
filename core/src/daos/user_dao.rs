use diesel::{QueryResult, PgConnection};
use crate::schema::users;
use crate::user::User;
use diesel::prelude::*;


pub fn insert_user(user: User, conn: &PgConnection) -> Result<User, diesel::result::Error> {
    let response: QueryResult<User> = diesel::insert_into(users::table)
    .values(user)
    .get_result(conn);
    response
}

pub fn get_user(username: String, conn: &PgConnection) -> Result<User, diesel::result::Error>{
    let response: QueryResult<User> = users::table.filter(users::username.eq(username)).first(conn);
    response
}