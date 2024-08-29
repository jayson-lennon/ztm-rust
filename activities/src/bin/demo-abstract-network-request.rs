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

trait DataFetcher {
    fn get_person(&self) -> Result<PersonResponse>;
}

#[derive(Debug, Clone, Default)]
struct ReqwestFetcher {
    client: reqwest::blocking::Client,
}

impl DataFetcher for ReqwestFetcher {
    fn get_person(&self) -> Result<PersonResponse> {
        let response: PersonResponse = self
            .client
            .get("https://fakerapi.it/api/v1/custom?_quantity=1&name=name&country=country")
            .send()?
            .json()?;

        Ok(response)
    }
}

struct App {
    fetcher: Box<dyn DataFetcher>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            fetcher: Box::new(ReqwestFetcher::default()),
        }
    }
}

impl App {
    pub fn fetch_person(&self) -> Result<PersonResponse> {
        self.fetcher.get_person()
    }
}

fn main() -> Result<()> {
    let app = App::default();
    let person = app.fetch_person()?;

    println!("{person:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestFetcher;

    impl DataFetcher for TestFetcher {
        fn get_person(&self) -> Result<PersonResponse> {
            Ok(PersonResponse {
                status: "OK".to_string(),
                code: 200,
                total: 1,
                data: vec![Person {
                    name: "test name".to_string(),
                    country: "test country".to_string(),
                }],
            })
        }
    }

    #[test]
    fn fetches_data() {
        let app = App {
            fetcher: Box::new(TestFetcher),
        };
        let person = app.fetch_person();
        assert!(person.is_ok());
    }
}
