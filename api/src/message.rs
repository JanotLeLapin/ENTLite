use crate::URL;
use reqwest::{blocking::Client, Error};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Message {
    pub id: String,
    pub date: u64,
    pub subject: String,
    pub parent_id: String,
    pub unread: bool,
    pub response: bool,
    pub from: String,
}

pub fn fetch_count(client: &Client, unread_only: bool) -> Result<u64, Error> {
    let res: Value = client.get(format!("{}/zimbra/count/INBOX?unread={}", URL, unread_only))
        .send()?
        .json()?;
    Ok(res["count"].as_u64().unwrap_or(0))
}

pub fn fetch_messages(client: &Client, folder: &str, page: usize, unread_only: bool) -> Result<Vec<Message>, Error> {
    Ok(client.get(format!("{}/zimbra/list?folder={}&page={}&unread={}", URL, folder, page, unread_only))
        .send()?
        .json()?
    )
}