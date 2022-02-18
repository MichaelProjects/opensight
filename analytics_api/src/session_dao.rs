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
    pub is_first_today: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Queryable, AsChangeset, Insertable)]
pub struct Session {
    pub id: String,
    application_id: String,
    pub length: i32,
    is_first_login_today: bool,
    pub start_time: NaiveDateTime,
}
impl Session {
    pub fn new(session_id: String, application_id: String, length: i32, start_time: NaiveDateTime, is_first_login_today: bool)-> Self {
        Session {
            id: session_id,
            application_id,
            length,
            is_first_login_today,
            start_time
        }
    }
    pub fn from_analytic_entry(entry: &AnalyticEntry) -> Self {
        Session {
            id: entry.session_id.clone(),
            application_id: entry.application_id.clone(),
            length: 0,
            is_first_login_today: false,
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

pub async fn update_session(id: String, new_length: i32, conn: AnalyticsDB, first_today: bool)-> QueryResult<Session>{
    let response = conn.run(move |c|
    diesel::update(sessions::table.filter(sessions::id.eq(id)))
    .set((sessions::length.eq(new_length), sessions::is_first_login_today.eq(first_today)))
    .get_result::<Session>(c)).await;
    return response
}

pub async fn get_sessions_in_timeframe(app_id: String, start_time: NaiveDateTime, end_time: NaiveDateTime, conn: AnalyticsDB)-> QueryResult<Vec<Session>> {
    println!("{}", start_time);
    println!("{}", end_time);
    let response = conn.run(move |c| 
        sessions::table.filter(sessions::application_id.eq(app_id))
    .filter(sessions::start_time.between(start_time, end_time))
    .load::<Session>(c)).await;
    return response;
}

pub async fn get_session_count_in_timeframe(app_id: String, start_time: NaiveDateTime, end_time: NaiveDateTime, conn: AnalyticsDB)-> QueryResult<Vec<Session>> {
    let response = conn.run(move |c| 
        sessions::table.filter(sessions::application_id.eq(app_id))
        .filter(sessions::start_time.between(start_time, end_time))
        .filter(sessions::is_first_login_today.eq(true))
        .load::<Session>(c)).await;
    return response
}