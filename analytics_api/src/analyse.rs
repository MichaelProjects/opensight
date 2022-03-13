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
    for day in days_vec.iter(){
        let mut key = day.clone();
        days.push(DayData::new(day.to_owned(), 0));
        loop{
            let mut entry = entry_iter.next();
            if entry.is_none(){
                break
            }
            println!("{:?}", entry);
            let entry = entry.expect("");
            let time = entry.creation_time.to_string();
            let time = time.split(" ").collect::<Vec<&str>>();
            println!("{:?}", time[0]);
            if time[0].ne(key.as_str()) {
                println!("Other key");
                days.last_mut()
                .expect("Could not get last element from vec!")
                .counter = counter;
                //todo slice vec only continue with the not processed ones
                counter = 0;
                continue
            }
            println!("INCREASE");
            counter+=1;
            println!("{}", counter);
        }

        // for entry in entrys.iter() {
        //     let time = entry.creation_time.to_string();
        //     let time = time.split(" ").collect::<Vec<&str>>();
        //     println!("{:?}", time[0]);
        //     if time[0].ne(key.as_str()) {
        //         println!("Other key");
        //         days.last_mut()
        //         .expect("Could not get last element from vec!")
        //         .counter = counter;
        //         //todo slice vec only continue with the not processed ones
        //         counter = 0;
        //         break
        //     }
        //     println!("INCREASE");
        //     counter+=1;
        //     println!("{}", counter);
        // }
    }
    // days.last_mut()
    //             .expect("Could not get last element from vec!")
    //             .counter = counter;



    // for entry in entrys.iter() {
    //     let time = entry.creation_time.to_string();
    //     let time = time.split(" ").collect::<Vec<&str>>();
    //     let key = time[0];
    //     if key.ne(before.as_str()) {
    //         days.push(DayData {
    //             day: key.to_string(),
    //             counter: 1,
    //         });
    //         before = key.to_string();
    //     } else {
    //         days.last_mut()
    //             .expect("Could not get last element from vec!")
    //             .counter += 1;
    //     }
    // }
    days
}

pub async fn sort_user_to_day<'a>(entrys: Vec<Session>, days: Vec<String>) -> Vec<DayData> {
    let mut days: Vec<DayData> = vec![];
    let mut before: String = String::new();
    
    for entry in entrys.iter() {
        let time = entry.start_time.to_string();
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

fn get_day_from_timestamp(timestamp_string: String) -> String{
    let time = timestamp_string.split(" ").collect::<Vec<&str>>();
    time[0].to_string()
}

pub async fn calc_average_session_length(data: Vec<Session>, days: Vec<String>) -> Vec<DayData>{
    let mut days: Vec<DayData> = vec![];
    let mut before: String = String::new();
    let mut session_counter = 0;
    println!("{:?}", &data.len());
    for session in data.iter(){
        let day = get_day_from_timestamp(session.start_time.to_string());
        if day.ne(before.as_str()) {
            if !days.is_empty(){
            // before adding a new day, divide the session counter by the session length overall
            let a = days.last_mut().expect("Could not get last element from vec!");
            a.counter = a.counter / session_counter;
            // reset counter
            session_counter = 0;
            }
            before = day.to_string();
            days.push(DayData {
                day,
                counter: session.length as i64});
        }else{
            // gets the last day and adds the session length.
            days.last_mut()
                .expect("Could not get last element from vec!")
                .counter += session.length as i64;
            session_counter += 1;
        }
    }

    // quick fix, if only one day or the last day in in data, the session length wont get divided, and returns a false value
    if !days.is_empty(){
        // before adding a new day, divide the session counter by the session length overall
        let a = days.last_mut().expect("Could not get last element from vec!");
        a.counter = a.counter / session_counter;
        }
    days
}

pub async fn version_analysis(data: Vec<AnalyticEntry>, days: Vec<String>) -> Vec<DayData>{
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
    use crate::analyse::{AnalyticEntry, sort_data_to_day, display_sizes, calc_average_session_length};

    fn get_test_dates(count: usize) -> Vec<String> {
        let a = vec![NaiveDateTime::from_timestamp(1646870400000 / 1000, 0).date().to_string(), NaiveDateTime::from_timestamp(1647093749737 / 1000, 0).date().to_string(), NaiveDateTime::from_timestamp(1647126000000 / 1000, 0).date().to_string()];
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
        let test2 = sort_data_to_day(get_data(), get_test_dates(3)).await;
        println!("{:?}", test2);
        assert_eq!(test2[1].counter, 2);
    }
    #[tokio::test]
    async fn test_device_size_func(){
        let data = get_data();
        let result = display_sizes(data).await;
        println!("{:?}", result);
    }
    #[tokio::test]
    async fn test_calc_average_session_length(){
        // let data = get_data();
        // let result = calc_average_session_length(data, get_test_dates()).await;
        // println!("{:?}", result);
    }
}
