extern crate diesel;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use diesel::{PgConnection};
use super::schema::analytics;
use crate::dao::Dao;
use crate::analytics_dao::AnalyticsDao;
use crate::application_dao::ApplicationDao;

#[derive(Deserialize)]
pub struct AnalyticData{
    #[serde(serialize_with = "to_ts")]
    pub creation_time: NaiveDateTime,
    pub os: String,
    pub device_size: String,
    pub new_user: bool,
    pub country: String,
    pub last_session: i32,
    pub device_type: String,
    pub version: String

}
#[derive(Serialize, Deserialize, Clone, Debug, Queryable, AsChangeset, Insertable)]
#[table_name="analytics"]
pub struct AnalyticEntry{
    session_id: String,
    application_id: String,
    creation_time: NaiveDateTime,
    os: String,
    device_size: String,
    new_user: bool,
    country: String,
    last_session: i32,
    device_type: String,
    version: String
    // here should come _ features: Vec<String>
}
impl AnalyticEntry{
    pub fn new(data: AnalyticData, application_id: String) -> Self{
        AnalyticEntry{
            session_id: create_tracking_id(),
            application_id,
            creation_time: data.creation_time,
            os: data.os,
            device_size: data.device_size,
            new_user: data.new_user,
            country: data.country,
            last_session: data.last_session,
            device_type: data.device_type,
            version: data.version
        }
    }
    pub fn insert_entry(self, conn: &mut PgConnection) -> bool{
        let dao = AnalyticsDao::new();
        dao.insert_entry(self, conn);
        let _successful = true;
        true
    }
}
pub fn get_all_entrys(conn: &mut PgConnection) -> Vec<AnalyticEntry>{
    let dao = AnalyticsDao::new();
    let entrys = dao.get_all(conn);
    return entrys;
}

pub fn create_tracking_id() -> String {
    let tracking_id = Uuid::new_v4();
    tracking_id.to_string()
}