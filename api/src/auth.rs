use crate::URL;
use std::collections::HashMap;
use reqwest::{ClientBuilder, Client, redirect::Policy, Error};

pub async fn login(email: &str, password: &str) -> Result<Client, Error> {
    let mut params = HashMap::new();
    params.insert("email", email);
    params.insert("password", password);
    params.insert("details", "/");
    params.insert("callBack", "https%3A%2F%2Fent.iledefrance.fr%2Ftimeline%2Ftimeline");

    let client = ClientBuilder::new()
        .redirect(Policy::none())
        .cookie_store(true)
        .build().unwrap();
    let _ = client.post(format!("{}/auth/login", URL))
        .form(&params)
        .send().await?;

    Ok(client)
}