use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Hash, Debug)]
enum ApplicationType { IOS, Android, Web }

#[derive(Hash, Debug)]
struct Application {
    name: String,
    os: ApplicationType,
    uuid: String,
    added: DateTime<Utc>,
    token: String,
}
impl Application{
    pub fn new(name: &str, os: ApplicationType) -> Application{
        let uuid: String = Uuid::new_v4().to_string();
        let get_time = Utc::now();
        let mut app = Application{name: String::from(name), os, uuid, added: get_time, token: String::new()};
        app.token = create_token(&app);
        app
    }
}

fn create_token(app: &Application) -> String{
    let mut s = DefaultHasher::new();
    app.hash(&mut s);
    s.finish().to_string()
}