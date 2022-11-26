use crate::URL;
use reqwest::{Client,Error};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Userbook {
    pub mood: String,
    pub health: String,
    #[serde(rename(serialize = "alertSize", deserialize = "alertSize"))]
    pub alert_size: bool,
    pub storage: usize,
    pub userid: String,
    pub picture: String,
    pub quota: usize,
    pub motto: String,
    pub theme: String,
}

pub async fn fetch_userbook(client: &Client, id: &str) -> Result<Userbook, Error> {
    let res = client.get(format!("{}/directory/userbook/{}", URL, id))
        .send().await?
        .json().await?;

    Ok(Userbook::from(res))
}