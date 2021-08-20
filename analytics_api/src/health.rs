use serde::{Deserialize, Serialize};
use log::{debug};

#[derive(Deserialize, Serialize)]
pub struct Health {
    pub error: bool,
    pub connection: String,
}

pub fn get_health_state() -> Health{
    debug!("response: {:?}", 12);
    Health{
        error: false, connection: String::from("Connected")
    }
}
