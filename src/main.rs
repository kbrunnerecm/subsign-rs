use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use config_file::FromConfigFile;
use reqwest::Client;
use serde::__private228::ser::constrain;
use crate::model::SubsignConfig;
use crate::sign::login;

mod sign;
mod model;







#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SubsignConfig::from_config_file("/home/kevin/.subsign/config.toml").unwrap();
    println!("{:#?}", config);
    let access_token = login(&config, &Client::new()).await?;
    Ok(())
}
