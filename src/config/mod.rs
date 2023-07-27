use serde::Deserialize;
use dotenv::dotenv;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Config {
    pub database_url: String,
}

pub fn load() -> Config {
    dotenv().ok();

    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    }
}
