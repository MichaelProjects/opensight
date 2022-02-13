use crate::analytics::AnalyticEntry;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DayData {
    pub day: String,
    pub counter: i64,
}

pub async fn display_sizes(entrys: Vec<AnalyticEntry>) -> Vec<i64> {
    let mut average_screen_size_height = 0;
    let mut average_screen_size_width = 0;
    let mut counter = 0;
    for entry in entrys.iter() {
        let display_string = entry.device_size.to_string();
        let splitted = display_string.split("x").collect::<Vec<&str>>();

        average_screen_size_width = average_screen_size_width + splitted[0].parse::<f64>().expect("could not parse value") as i64;
        average_screen_size_height =
            average_screen_size_height + splitted[1].parse::<f64>().unwrap() as i64;
        counter += 1;
    }
    vec![
        average_screen_size_height / counter,
        average_screen_size_width / counter,
    ]
}

pub async fn sort_data_to_day<'a>(entrys: Vec<AnalyticEntry>) -> Vec<DayData> {
    let mut days: Vec<DayData> = vec![];
    let mut before: String = String::new();
    for entry in entrys.iter() {
        let time = entry.creation_time.to_string();
        let time = time.split(" ").collect::<Vec<&str>>();
        let key = time[0];
        if key.ne(before.as_str()) {
            days.push(DayData {
                day: key.to_string(),
                counter: 1,
            });
            before = key.to_string();
        } else {
            days.last_mut()
                .expect("Could not get last element from vec!")
                .counter += 1;
        }
    }
    days
}

mod tests {
    use rocket::tokio;
    use chrono::naive::NaiveDateTime;
    use crate::analyse::{AnalyticEntry, sort_data_to_day, display_sizes};
    fn get_data() -> Vec<AnalyticEntry> {
        let raw_data = vec![
            AnalyticEntry {
                session_id: "1".to_string(),
                application_id: "1".to_string(),
                creation_time: NaiveDateTime::parse_from_str("2022-02-01T19:26:37", "%Y-%m-%dT%H:%M:%S").unwrap(),
                os: "1".to_string(),
                device_size: "1.0".to_string(),
                new_user: true,
                country: "1".to_string(),
                last_session: 1,
                device_type: "1".to_string(),
                version: "1".to_string(),
            },
            AnalyticEntry {
                session_id: "1".to_string(),
                application_id: "1".to_string(),
                creation_time: NaiveDateTime::parse_from_str("2022-02-02T19:26:37", "%Y-%m-%dT%H:%M:%S").unwrap(),
                os: "1".to_string(),
                device_size: "1".to_string(),
                new_user: true,
                country: "1".to_string(),
                last_session: 1,
                device_type: "1".to_string(),
                version: "1".to_string(),
            },
        ];
        raw_data
    }
    
    #[tokio::test]
    async fn test_analyse() {
        use chrono::NaiveDateTime;
        let parse_from_str = NaiveDateTime::parse_from_str;
        
        let data = sort_data_to_day(get_data()).await;
        assert_eq!(data.len(), 2);
    }
    #[tokio::test]
    async fn test_device_size_func(){
        let data = get_data();
        let result = display_sizes(data).await;
        println!("{:?}", result);
    }
}
