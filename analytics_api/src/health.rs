use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Health {
    pub error: bool,
    pub connection: String,
}

pub fn get_health_state() -> Health{
    println!("response: {:?}", 12);
    Health{
        error: false, connection: String::from("Connected")
    }
}
