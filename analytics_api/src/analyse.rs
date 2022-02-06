use std::{ops::Add};
use serde::Serialize;
use crate::analytics::AnalyticEntry;

#[derive(Debug, Clone, Serialize)]
pub struct DayData{
    pub day: String,
    pub counter: i64
}

pub fn sort_data_to_day<'a>(entrys: Vec<AnalyticEntry>) -> Vec<DayData> {
    let mut days: Vec<DayData> = vec![];
    let mut before: String = String::new();
    for entry in entrys.iter(){
        let time = entry.creation_time.to_string();
        let time = time.split("T").collect::<Vec<&str>>();
        let key= time[0];
        if key.ne(before.as_str()){
                days.push(DayData{day: key.to_string(), counter: 1});
                before = key.to_string();
            }
            else {
                let a = days.last_mut().expect("Could not get last element from vec!").counter.add(1);
            }
        }
    days
}

#[test]
fn test_analyse(){
    use chrono::NaiveDateTime;
    let parse_from_str = NaiveDateTime::parse_from_str;
    let raw_data = vec![AnalyticEntry{
        session_id: "1".to_string(),
        application_id: "1".to_string(),
        creation_time: parse_from_str("2022-02-01T19:26:37" , "%Y-%m-%dT%H:%M:%S").unwrap(),
        os: "1".to_string(),
        device_size: "1".to_string(),
        new_user: true,
        country: "1".to_string(),
        last_session: 1,
        device_type: "1".to_string(),
        version: "1".to_string(),
    },
    AnalyticEntry{
        session_id: "1".to_string(),
        application_id: "1".to_string(),
        creation_time: parse_from_str("2022-02-02T19:26:37" , "%Y-%m-%dT%H:%M:%S").unwrap(),
        os: "1".to_string(),
        device_size: "1".to_string(),
        new_user: true,
        country: "1".to_string(),
        last_session: 1,
        device_type: "1".to_string(),
        version: "1".to_string(),
    }];
    let data = sort_data_to_day(raw_data);
    assert_eq!(data.len(), 2);
}