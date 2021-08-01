use serde::{Deserialize, Serialize};
use postgres::{Client};

#[derive(Deserialize, Serialize)]
pub struct Health {
    pub error: bool,
    pub connection: String,
}

pub fn get_health_state(_connection_str: String) -> Health{
    println!("response: {:?}", 12);
    Health{
        error: false, connection: String::from("Connected")
    }
}
