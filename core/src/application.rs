use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Clone, )]
pub struct Application {
    application_name: String,
    creation_time: NaiveDateTime,
    token: String,
    os: String,
}