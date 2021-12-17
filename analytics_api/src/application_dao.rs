use reqwest;
use std::error::Error;
use crate::settings::Settings;

pub async fn get_application(conf: &Settings, application_id: &String) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(conf.general.opensight_core.as_str()).await?;
    let a = response.text().await?;
    Ok(a)
}

