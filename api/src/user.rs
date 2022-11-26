use crate::URL;
use reqwest::{blocking::Client, Error};
use serde::Deserialize;
use serde_json::Value;

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

#[derive(Deserialize, Debug)]
pub struct Person {
    pub id: String,
    pub login: String,
    #[serde(rename(serialize = "displayName", deserialize = "displayName"))]
    pub display_name: String,
}

#[derive(Deserialize, Debug)]
pub struct Userinfo {
    #[serde(rename(serialize = "userId", deserialize = "userId"))]
    pub id: String,
    pub level: String,
    #[serde(rename(serialize = "lastName", deserialize = "lastName"))]
    pub last_name: String,
    #[serde(rename(serialize = "firstName", deserialize = "firstName"))]
    pub first_name: String,
}

pub fn fetch_userbook(client: &Client, id: &str) -> Result<Userbook, Error> {
    Ok(client.get(format!("{}/directory/userbook/{}", URL, id))
        .send()?
        .json()?
    )
}

pub fn fetch_person(client: &Client, id: &str) -> Result<Person, Error> {
    let res: Value = client.get(format!("{}/userbook/api/person?id={}", URL, id))
        .send()?
        .json()?;

    Ok(serde_json::from_value(res["result"].as_array().unwrap()[0].clone()).unwrap())
}

pub fn fetch_userinfo(client: &Client) -> Result<Userinfo, Error> {
    Ok(client.get(format!("{}/auth/oauth2/userinfo", URL))
        .send()?
        .json()?
    )
}