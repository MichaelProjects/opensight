use reqwest;
use serde::Deserialize;
use std::{error::Error, collections::HashMap, io::ErrorKind};
use crate::{settings::Settings, application::Application};



pub async fn get(conf: &Settings, application_id: &String) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(conf.general.opensight_core.as_str()).await?;
    let a = response.text().await?;
    Ok(a)
}
#[derive(Debug, Deserialize)]
struct ApiResponse{
    error: bool,
    data: Vec<Application>
}

pub async fn get_all(conf: &Settings) -> Result<Vec<Application>, Box<dyn Error>> {
    let url: String = format!("{}/core/v1/application", conf.general.opensight_core);
    let response = reqwest::get(url).await?;
    let api_response: ApiResponse = response.json().await?;
    if api_response.error.eq(&true){
        let a = api_response.data;
        Ok(a)
    }
}