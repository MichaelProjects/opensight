use serde::Deserialize;
use uuid::Uuid;
use postgres::{Client};
use chrono::{DateTime, Utc};


#[derive(Deserialize)]
pub struct AnalyticData{
    #[serde(serialize_with = "to_ts")]
    pub creation_date: DateTime<Utc>,
    pub os: String,
    pub device_size: String,
    pub session_length: i64,
    pub session_id: String

}
#[derive(Clone, Debug)]
pub struct AnalyticEntry{
    tracking_id: String,
    application_id: String,
    creation_time: DateTime<Utc>,
    os: String,
    device_size: String,
    session_length: i64,
    session_id: String
    // here should come _ features: Vec<String>
}
impl AnalyticEntry{
    pub fn new(application_id: String, creation_time: DateTime<Utc>, os: String, device_size: String, session_id: String, session_length: i64) -> Self{
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
    pub fn insert_entry(self, conn: &mut Client) -> bool{
        let mut successful = true;
        let query = "INSERT INTO analytics (tracking_id, application_id, creation_time, os, device_size, session_length, session_id) values ($1,$2,$3,$4,$5,$6,$7)".to_string();
        let response = match conn.execute(query.as_str(),&[&self.tracking_id, &self.application_id, &self.creation_time, &self.os, &self.device_size, &self.session_length, &self.session_id]){
            Ok(response) => response,
            Err(err) => panic!("Error while inserting: {}", err)
        };
        println!("Rows Affected: {}", response);
        if response == 0{
            successful = false;
        }
        successful
    }
}

pub fn create_tracking_id() -> String {
    let tracking_id = Uuid::new_v4();
    tracking_id.to_string()
}