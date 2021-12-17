use reqwest;
use std::{error::Error, collections::HashMap};
use crate::{settings::Settings, application::Application};



pub async fn get(conf: &Settings, application_id: &String) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(conf.general.opensight_core.as_str()).await?;
    let a = response.text().await?;
    Ok(a)
}
struct AllAppPayload{
    error: bool,
    data: Vec<Application>
}

pub async fn get_all(conf: &Settings) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let url: String = format!("{}/core/v1/application", conf.general.opensight_core);
    let response = reqwest::get(url).await?;
    let a: AllAppPayload = response.json().await?;
    Ok(a)
}