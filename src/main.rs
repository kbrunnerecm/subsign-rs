use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use config_file::FromConfigFile;
use reqwest::Client;
use reqwest::redirect::Policy;
use serde::__private228::ser::constrain;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::model::SubsignConfig;
use crate::sign::login;

mod sign;
mod model;







#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "reqwest=trace,hyper=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


    let config = SubsignConfig::from_config_file("/home/kevin/.subsign/config.toml").unwrap();
    println!("{:#?}", config);
    let client = Client::builder().redirect(Policy::limited(3)).build()?;
    let access_token = login(&config, &client).await?;
    Ok(())
}
