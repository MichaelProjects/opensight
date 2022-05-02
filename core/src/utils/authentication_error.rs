use std::fmt;
use std::error;


type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub enum LoginError{ NotFound, Failed }

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         match self {
            LoginError::NotFound => write!(f, "Could not find the corresponding user"),
            LoginError::Failed => write!(f, "Data operation failed"),
        }
    }
}