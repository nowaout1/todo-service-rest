use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Config {
    pub addr: &'static str,
    pub db_url: &'static str,
}

impl Config {
    pub fn parse() -> Self {
        let addr = dotenvy::var("ADDR").expect("environment variable `ADDR` was not specified");
        let db_url =
            dotenvy::var("ADDR").expect("environment variable `DATABASE_URL` was not specified");

        Self {
            addr: addr.leak(),
            db_url: db_url.leak(),
        }
    }
}
