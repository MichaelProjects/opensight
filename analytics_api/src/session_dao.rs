extern crate diesel;

use chrono::{NaiveDateTime};
use diesel::{QueryResult, Insertable};
use serde::{Deserialize, Serialize};
use crate::{db::AnalyticsDB, analytics::AnalyticEntry};
use super::schema::sessions;

use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, BoolExpressionMethods};

#[derive(Deserialize, Debug)]
pub struct SessionIn{
    pub session_id: String,
    pub length: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Queryable, AsChangeset, Insertable)]
pub struct Session {
    pub id: String,
    application_id: String,
    length: i32,
    start_time: NaiveDateTime,
}
impl Session {
    pub fn new(session_id: String, application_id: String, length: i32, start_time: NaiveDateTime)-> Self {
        Session {
            id: session_id,
            application_id,
            length,
            start_time
        }
    }
    pub fn from_analytic_entry(entry: &AnalyticEntry) -> Self {
        Session {
            id: entry.session_id.clone(),
            application_id: entry.application_id.clone(),
            length: 0,
            start_time: entry.creation_time,
        }
    }
}



pub fn create_session(data: Session, conn: &mut PgConnection) -> QueryResult<Session>{
    let response: QueryResult<Session> =
        diesel::insert_into(sessions::table)
        .values(&data)
        .get_result(conn);
    return response;
        
}

pub async fn get_session(session_id: String, conn: AnalyticsDB){
    let reponse = conn.run(move |c| 
        sessions::table
        .filter(sessions::id.eq(session_id))
        .load::<Session>(c)).await;
}

pub async fn update_session(id: String, new_length: i32, conn: AnalyticsDB)-> QueryResult<Session>{
    let response = conn.run(move |c|
    diesel::update(sessions::table.filter(sessions::id.eq(id)))
    .set(sessions::length.eq(new_length))
    .get_result::<Session>(c)).await;
    return response
}
