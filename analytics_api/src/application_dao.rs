use reqwest;
use rocket::tokio;
use serde::Deserialize;
use serde_json::Value;
use std::{error::Error, collections::HashMap, io::ErrorKind};
use crate::{settings::Settings, application::Application};



pub async fn get(conf: &Settings, application_id: &String) -> Result<Application, Box<dyn Error>> {
    let response = reqwest::get(conf.general.opensight_core.as_str()).await?;
    let data: Application = response.json().await?;
    Ok(data)
}
#[derive(Debug, Deserialize)]
struct ApiResponse{
    error: Value,
    data: Vec<Application>
}

pub async fn get_all(host: &String) -> Result<Vec<Application>, Box<dyn Error>> {
    let url: String = format!("{}/core/v1/application", host);
    let response = reqwest::get(url).await?;
    let api_response: ApiResponse = response.json().await?;
    Ok(api_response.data)
}

#[tokio::test]
async fn get_all_test(){
    let url ="http://metrics.stackblog.io";
    let result = get_all(&url.to_string()).await.unwrap();
    
}