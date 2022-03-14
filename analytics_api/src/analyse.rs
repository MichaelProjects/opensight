use crate::{analytics::AnalyticEntry};
use crate::daos::session_dao::Session;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DayData {
    pub day: String,
    pub counter: i64,
}
impl DayData{
    pub fn new(day:  String, counter: i64) -> DayData{
        DayData{day, counter}
    }
}


// todo need to be fixed, there is currently an issue
pub async fn display_sizes(entrys: Vec<AnalyticEntry>) -> Vec<i64> {
    let mut average_screen_size_height = 0;
    let mut average_screen_size_width = 0;
    let mut counter = 0;
    for entry in entrys.iter() {
        let display_string = entry.device_size.to_string();
        let splitted = display_string.split("x").collect::<Vec<&str>>();

        average_screen_size_width = average_screen_size_width + splitted[0].parse::<f64>().expect("could not parse value") as i64;
        average_screen_size_height =
            average_screen_size_height + splitted[0].parse::<f64>().unwrap() as i64;
        counter += 1;
    }
    vec![
        average_screen_size_height / counter,
        average_screen_size_width / counter,
    ]
}

pub async fn sort_data_to_day<'a>(entrys: Vec<AnalyticEntry>, days_vec: Vec<String>) -> Vec<DayData> {
    let mut days: Vec<DayData> = vec![];
    let mut counter = 0;
    let mut entry_iter = entrys.iter();
    
    // if a new value gets called with entry_iter.next() it gets removed, if the key is not the same as the next day it will be store here.
    let mut before_vaule = String::new();
    
    for day in days_vec.iter(){
        let mut key = day.clone();
        days.push(DayData::new(day.to_owned(), 0));
        loop{

            // if current key is the same as before_value
            if before_vaule.ne(&String::new()){
                if key.eq(&before_vaule){
                    counter +=1;
                    before_vaule = String::new();
                }else{
                    break
                }
            }

            let mut entry = entry_iter.next();
            // if iterator has next value
            if entry.is_none(){
                // if the last day has data but isnt able to reach the end, the before it end it takes the count and replace the last day count in days vector.
                if counter.ne(&0){
                    days.last_mut()
                                 .expect("Could not get last element from vec!")
                                 .counter = counter;
                    counter = 0;
                }
                break
            }

            log::debug!("{:?}", entry);
            let entry = entry.expect("");
            
            // time splitting to key to determine if current entry date is the sam of the current day
            let time = entry.creation_time.to_string();
            let time = time.split(" ").collect::<Vec<&str>>();
            log::debug!("{:?}", time[0]);
            
            
            // if the current day is not the same replace the counter value of the current day with the current counter and set global counter to 0 
            if time[0].ne(key.as_str()) {
                log::debug!("Other key");
                days.last_mut()
                .expect("Could not get last element from vec!")
                .counter = counter;
                //todo slice vec only continue with the not processed ones
                before_vaule = time[0].to_string();
                counter = 0;
                break
            }

            log::debug!("INCREASE");
            counter+=1;
            log::debug!("{}", counter);
        }
    }
    days
}

pub async fn sort_user_to_day<'a>(entrys: Vec<Session>, days_vec: Vec<String>) -> Vec<DayData> {
    let mut days: Vec<DayData> = vec![];
    let mut counter = 0;
    let mut entry_iter = entrys.iter();
    
    // if a new value gets called with entry_iter.next() it gets removed, if the key is not the same as the next day it will be store here.
    let mut before_vaule = String::new();
    
    for day in days_vec.iter(){
        let mut key = day.clone();
        days.push(DayData::new(day.to_owned(), 0));
        loop{

            // if current key is the same as before_value
            if before_vaule.ne(&String::new()){
                if key.eq(&before_vaule){
                    counter +=1;
                    before_vaule = String::new();
                }else{
                    break
                }
            }

            let mut  entry = entry_iter.next();
            // if iterator has next value
            if entry.is_none(){
                // if the last day has data but isnt able to reach the end, the before it end it takes the count and replace the last day count in days vector.
                if counter.ne(&0){
                    days.last_mut()
                                 .expect("Could not get last element from vec!")
                                 .counter = counter;
                    counter = 0;
                }
                break
            }

            log::debug!("{:?}", entry);
            let entry = entry.expect("");
            
            // time splitting to key to determine if current entry date is the sam of the current day
            let time = entry.start_time.to_string();
            let time = time.split(" ").collect::<Vec<&str>>();
            log::debug!("{:?}", time[0]);
            
            
            // if the current day is not the same replace the counter value of the current day with the current counter and set global counter to 0 
            if time[0].ne(key.as_str()) {
                log::debug!("Other key");
                days.last_mut()
                .expect("Could not get last element from vec!")
                .counter = counter;
                //todo slice vec only continue with the not processed ones
                before_vaule = time[0].to_string();
                counter = 0;
                break
            }

            log::debug!("INCREASE");
            counter+=1;
            log::debug!("{}", counter);
        }
    }
    days
}

fn get_day_from_timestamp(timestamp_string: String) -> String{
    let time = timestamp_string.split(" ").collect::<Vec<&str>>();
    time[0].to_string()
}



pub async fn calc_average_session_length(entrys: Vec<Session>, days_vec: Vec<String>) -> Vec<DayData>{
    let mut days: Vec<DayData> = vec![];
    let mut entry_iter = entrys.iter();
    
    // if a new value gets called with entry_iter.next() it gets removed, if the key is not the same as the next day it will be store here.
    let mut before_vaule = String::new();
    let mut before_session_length: i64 = 0;
    
    for day in days_vec.iter(){
        let mut key = day.clone();
        let mut sum_session_length = 0;
        let mut counter = 0;
        days.push(DayData::new(day.to_owned(), 0));
        loop{

            // if current key is the same as before_value
            if before_vaule.ne(&String::new()){
                if key.eq(&before_vaule){
                    counter +=1;
                    sum_session_length += before_session_length;
                    before_session_length = 0;
                    before_vaule = String::new();
                }else{
                    break
                }
            }

            let mut entry = entry_iter.next();
            // if iterator has next value
            if entry.is_none(){
                // if the last day has data but isnt able to reach the end, the before it end it takes the count and replace the last day count in days vector.
                if counter.ne(&0){
                    days.last_mut()
                                 .expect("Could not get last element from vec!")
                                 .counter = sum_session_length / counter;
                }
                break
            }

            log::debug!("{:?}", entry);
            let entry = entry.expect("");
            
            // time splitting to key to determine if current entry date is the sam of the current day
            let time = entry.start_time.to_string();
            let time = time.split(" ").collect::<Vec<&str>>();
            log::debug!("{:?}", time[0]);
            
            
            // if the current day is not the same replace the counter value of the current day with the current counter and set global counter to 0 
            if time[0].ne(key.as_str()) {

                log::debug!("Other key");
                println!("sum: {:?} count: {:?}", sum_session_length, counter);
                if counter.eq(&0){
                    days.last_mut()
                .expect("Could not get last element from vec!")
                .counter = 0;
                }else{
                    days.last_mut()
                .expect("Could not get last element from vec!")
                .counter = sum_session_length / counter;
                }
                before_vaule = time[0].to_string();
                before_session_length = entry.length as i64;
                counter = 0;
                sum_session_length = 0;
                break
            }

            log::debug!("INCREASE");
            sum_session_length += entry.length as i64;
            counter += 1;
            log::debug!("{}", counter);
        }
    }
    days
}


pub async fn version_analysis(data: Vec<AnalyticEntry>) -> Vec<DayData>{
    let mut result: Vec<DayData> = vec![];
    let mut versions = Vec::new();
    for entry in data.iter(){
        if !versions.contains(&entry.version){
            versions.push(entry.version.clone());
            result.push(DayData{
                day: entry.version.clone(),
                counter: 1,
            });
        }
        else{
            for i in 0..result.len(){
                if result[i].day == entry.version{
                    result[i].counter += 1;
                }
            }
        }
    }
    result
}

mod tests {
    use rocket::{tokio, http::ext::IntoCollection};
    use chrono::naive::{NaiveDateTime, NaiveDate};
    use crate::daos::session_dao::Session;
    use crate::analyse::{AnalyticEntry, sort_data_to_day, display_sizes, calc_average_session_length};

    fn get_test_dates(count: usize) -> Vec<String> {
        let a = vec![NaiveDateTime::from_timestamp(1646870400000 / 1000, 0).date().to_string(), NaiveDateTime::from_timestamp(1646956800000 / 1000, 0).date().to_string(), NaiveDateTime::from_timestamp(1647126000000 / 1000, 0).date().to_string(), NaiveDateTime::from_timestamp(1647256764000 / 1000, 0).date().to_string()];
        if count < 3{
            let mut x: Vec<String> = vec![];
            for y in a.iter(){
                if x.len().ne(&count) {
                    x.push(y.to_owned());
                }else{
                    return x;
                }
            }
        }
        a
    }

    fn get_data() -> Vec<AnalyticEntry> {
        let raw_data = vec![
            AnalyticEntry {
                session_id: "1".to_string(),
                application_id: "1".to_string(),
                creation_time: NaiveDateTime::parse_from_str("2022-03-10T10:33:28", "%Y-%m-%dT%H:%M:%S").unwrap(),
                os: "1".to_string(),
                device_size: "100x100".to_string(),
                new_user: true,
                country: "1".to_string(),
                device_type: "1".to_string(),
                version: "1".to_string(),
            },
            AnalyticEntry {
                session_id: "1".to_string(),
                application_id: "1".to_string(),
                creation_time: NaiveDateTime::parse_from_str("2022-03-12T10:33:28", "%Y-%m-%dT%H:%M:%S").unwrap(),
                os: "1".to_string(),
                device_size: "100x100".to_string(),
                new_user: true,
                country: "1".to_string(),
                device_type: "1".to_string(),
                version: "1".to_string(),
            },
            AnalyticEntry {
                session_id: "1".to_string(),
                application_id: "1".to_string(),
                creation_time: NaiveDateTime::parse_from_str("2022-03-12T10:33:28", "%Y-%m-%dT%H:%M:%S").unwrap(),
                os: "1".to_string(),
                device_size: "100x200".to_string(),
                new_user: true,
                country: "1".to_string(),
                device_type: "1".to_string(),
                version: "1".to_string(),
            },
        ];
        raw_data
    }
    fn get_session_data() -> Vec<Session>{
        let mut data = vec![];
        for x in get_data(){
            let mut y = Session::from_analytic_entry(&x);
            y.length = 100;
            data.push(y);
        }
        data
    }
    #[tokio::test]
    async fn test_analyse_one_item() {
        //todo test edge cases like what appends if only one entry is there or 3 or 2?
        use chrono::NaiveDateTime;
        let parse_from_str = NaiveDateTime::parse_from_str;

        let test1 = sort_data_to_day(get_data(), get_test_dates(1)).await;
        println!("{:?}", test1);
        assert_eq!(test1[0].counter, 1);
    }

    #[tokio::test]
    async fn test_analyse_multiple_item() {
        //todo test edge cases like what appends if only one entry is there or 3 or 2?
        use chrono::NaiveDateTime;
        let parse_from_str = NaiveDateTime::parse_from_str;
        // edge case with one day
        let days = get_test_dates(3);
        println!("{:?}", days);
        let test2 = sort_data_to_day(get_data(), get_test_dates(4)).await;
        println!("{:?}", test2);
        assert_eq!(test2[2].counter, 2);
        assert_eq!(test2[3].counter, 0);
    }
    #[tokio::test]
    async fn test_device_size_func(){
        let data = get_data();
        let result = display_sizes(data).await;
        println!("{:?}", result);
    }
    #[tokio::test]
    async fn test_calc_average_session_length(){
        let result = calc_average_session_length(get_session_data(), get_test_dates(3)).await;
        println!("{:?}", result);
        assert_eq!(result[2].counter, 100);
    }
}
