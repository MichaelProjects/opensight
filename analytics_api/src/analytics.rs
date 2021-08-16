extern crate diesel;

use serde::Deserialize;
use uuid::Uuid;
use chrono::{Utc, NaiveDateTime};
use diesel::{PgConnection};
use super::schema::analytics;

#[derive(Deserialize)]
pub struct AnalyticData{
    #[serde(serialize_with = "to_ts")]
    pub creation_date: NaiveDateTime,
    pub os: String,
    pub device_size: String,
    pub session_length: i32,
    pub session_id: String

}
#[derive(Clone, Debug, Queryable, Insertable)]
#[table_name="analytics"]
pub struct AnalyticEntry{
    tracking_id: String,
    application_id: String,
    creation_time: NaiveDateTime,
    os: String,
    device_size: String,
    session_length: i32,
    session_id: String
    // here should come _ features: Vec<String>
}
impl AnalyticEntry{
    pub fn new(application_id: String, creation_time: NaiveDateTime, os: String, device_size: String, session_id: String, session_length: i32) -> Self{
        error!("{:?}", Utc::now(),);
        error!("{:?}", creation_time);
        AnalyticEntry{
            tracking_id: create_tracking_id(),
            application_id,
            creation_time,
            os,
            device_size,
            session_length,
            session_id
        }
    }
    pub fn insert_entry(self, conn: &mut PgConnection) -> bool{
        let mut successful = true;
        true
    }
}

pub fn create_tracking_id() -> String {
    let tracking_id = Uuid::new_v4();
    tracking_id.to_string()
}