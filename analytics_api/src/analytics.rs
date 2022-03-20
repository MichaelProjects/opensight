extern crate diesel;

use std::error::Error;

use super::schema::analytics;
use crate::daos::session_dao::Session;
use crate::daos::{analytics_dao, session_dao};
use crate::daos::{analytics_dao::AnalyticsDao};

use crate::dao::Dao;
use crate::db::AnalyticsDB;
use chrono::NaiveDateTime;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AnalyticData {
    #[serde(serialize_with = "to_ts")]
    pub creation_time: NaiveDateTime,
    pub os: String,
    pub device_size: String,
    pub new_user: bool,
    pub country: String,
    pub device_type: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Queryable, AsChangeset, Insertable)]
#[table_name = "analytics"]
pub struct AnalyticEntry {
    pub session_id: String,
    pub application_id: String,
    pub creation_time: NaiveDateTime,
    pub os: String,
    pub device_size: String,
    pub new_user: bool,
    pub country: String,
    pub device_type: String,
    pub version: String, // here should come _ features: Vec<String>
}
impl AnalyticEntry {
    pub fn new(data: AnalyticData, application_id: String) -> Self {
        AnalyticEntry {
            session_id: Uuid::new_v4().to_string(),
            application_id,
            creation_time: data.creation_time,
            os: data.os,
            device_size: data.device_size,
            new_user: data.new_user,
            country: data.country,
            device_type: data.device_type,
            version: data.version,
        }
    }
}

pub async fn insert_entry(data: AnalyticEntry , session_data: Session, conn: AnalyticsDB) -> Result<Value, Box<dyn Error>> {
    let result =  conn.run(|c| analytics_dao::insert_entry(data, c)).await?;
    let response = conn.run(|c| session_dao::create_session(session_data, c)).await?;
    Ok(json!({"session_id": response.id}))
}

pub async fn get_all_entries(app_id: String, conn: AnalyticsDB, limit: i64, start: NaiveDateTime, end: NaiveDateTime) -> Vec<AnalyticEntry> {
    let entrys = analytics_dao::get_all(app_id, conn, limit, start, end).await;
    return entrys;
}

pub async fn get_timeframe_entries(app_id: String, conn: AnalyticsDB, start: NaiveDateTime, end: NaiveDateTime) -> Result<Vec<AnalyticEntry>, Box<dyn Error>> {
    let result = analytics_dao::get_timeframe_entry(app_id, conn, start, end).await?;
    Ok(result)
}

pub async fn get_newuser_in_timeframe(app_id: String, conn: AnalyticsDB, start: NaiveDateTime, end: NaiveDateTime) -> Result<Vec<AnalyticEntry>, Box<dyn Error>> {
    let result = analytics_dao::get_newuser_in_timeframe(app_id, conn, start, end).await?;
    Ok(result)
}

