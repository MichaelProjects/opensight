extern crate diesel;

use std::error::Error;

use super::schema::analytics;
use crate::analytics_dao;
use crate::{analytics_dao::AnalyticsDao, db::AnalyticsDB};
use crate::dao::Dao;
use chrono::NaiveDateTime;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AnalyticData {
    #[serde(serialize_with = "to_ts")]
    pub creation_time: NaiveDateTime,
    pub os: String,
    pub device_size: String,
    pub new_user: bool,
    pub country: String,
    pub last_session: i32,
    pub device_type: String,
    pub version: String,
}
#[derive(Deserialize, Debug)]
pub struct SessionUpdate {
    pub session_id: String,
    pub session_length: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Queryable, AsChangeset, Insertable)]
#[table_name = "analytics"]
pub struct AnalyticEntry {
    session_id: String,
    application_id: String,
    pub creation_time: NaiveDateTime,
    os: String,
    device_size: String,
    new_user: bool,
    country: String,
    last_session: i32,
    device_type: String,
    version: String, // here should come _ features: Vec<String>
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
            last_session: data.last_session,
            device_type: data.device_type,
            version: data.version,
        }
    }
    pub fn insert_entry(self, conn: &mut PgConnection) -> bool {
        let dao = AnalyticsDao::new();
        dao.insert_entry(self, conn);
        let _successful = true;
        true
    }
}

pub async fn get_all_entries(app_id: String, conn: AnalyticsDB) -> Vec<AnalyticEntry> {
    let entrys = analytics_dao::get_all(app_id, conn).await;
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