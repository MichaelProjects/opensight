use chrono::{NaiveDateTime, Utc, NaiveTime, NaiveDate};

pub fn calc_days_in_timeframe(start: &NaiveDateTime, end: &NaiveDateTime) -> Vec<String>{
    println!("{}", start);
    let day = start.date();
    let mut days = vec![];
    println!("{}", &day);
    for x in day.iter_days(){
        let y = x.and_time(NaiveTime::from_num_seconds_from_midnight(0, 0));
        if &y < end{
            days.push(x.to_string());
        }   
    }
    days
}


#[test]
fn test_calc_days_in_timeframe(){

    let days = calc_days_in_timeframe(&NaiveDateTime::from_timestamp(1646866800, 0), &NaiveDateTime::from_timestamp(1646914359, 0));
    for x in days.iter(){
        println!("{}", x);
    }
    assert_eq!(days.len(), 2);
}