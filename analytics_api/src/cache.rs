use crate::application::Application;
use std::sync::Mutex;

pub struct Cache{
    pub all_apps: Vec<Application>
}