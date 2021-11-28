use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Health {
    pub error: bool,
    pub connection: String,
}

pub fn get_health_state() -> Health {
    debug!("response: {:?}", 12);
    Health {
        error: false,
        connection: String::from("Connected"),
    }
}
