use crate::application::Application;
use diesel;
use super::schema::applications;
use diesel::{PgConnection, QueryResult, RunQueryDsl};
use diesel::prelude::*;


pub fn insert_application(application: Application, conn: &PgConnection) -> Result<Application, diesel::result::Error> {
    let response: QueryResult<Application> = diesel::insert_into(applications::table)
    .values(application)
    .get_result(conn);
    response
}

pub fn get_application(id: String, conn: &PgConnection) -> Result<Application, diesel::result::Error> {
    let response: QueryResult<Application> = applications::table.filter(applications::application_id.eq(id)).get_result(conn);
    response
}

pub fn get_all_applications(conn: &PgConnection) -> Vec<Application> {
    let response: QueryResult<Vec<Application>> = applications::table.load::<Application>(conn);
    return response.unwrap();
}