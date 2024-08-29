#![allow(dead_code)]

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Person {
    name: String,
    country: String,
}

#[derive(Debug, Clone, Deserialize)]
struct PersonResponse {
    status: String,
    code: u16,
    total: u64,
    data: Vec<Person>,
}

fn get_person(client: &reqwest::blocking::Client) -> Result<PersonResponse> {
    let response: PersonResponse = client
        .get("https://fakerapi.it/api/v1/custom?_quantity=1&name=name&country=country")
        .send()?
        .json()?;

    Ok(response)
}

#[derive(Debug, Default)]
struct App {
    client: reqwest::blocking::Client,
}

impl App {
    pub fn fetch_person(&self) -> Result<PersonResponse> {
        get_person(&self.client)
    }
}

fn main() -> Result<()> {
    Ok(())
}
