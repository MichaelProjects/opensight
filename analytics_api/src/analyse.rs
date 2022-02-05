use std::{collections::HashMap, ops::Add};

use crate::analytics::AnalyticEntry;

#[derive(Debug, Clone)]
struct DayData{
    day: String,
    counter: i64
}

pub fn analyse_user<'a>(entrys: Vec<AnalyticEntry>) -> Vec<DayData> {
    let mut days: Vec<DayData> = vec![];
    let mut before:String = String::new();
    for entry in entrys.iter(){
        let time = entry.creation_time.to_string().split("T").collect::<Vec<&str>>();
        let key= time[0];
        let x = &before.clone();
        match &mut before{
            _ if key.eq(x.as_str()) => {
                days.push(DayData{day: key.to_string(), counter: 1});
                key.to_string();
            }
            _ => {
                &mut days[&days.len()-1].counter.add(1);
                before;
            }
        }
    }
    days
}