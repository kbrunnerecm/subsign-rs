use std::fmt::format;
use std::io::{Error, ErrorKind};
use log::{debug, info};
use reqwest::{Client, Response};
use crate::model::{LoginResponse, SubsignConfig};

pub async  fn login(config: &SubsignConfig,client: &Client) -> Result<LoginResponse,Box<dyn std::error::Error>> {
    info!("Try to login");
    let url = format!("{}{}",config.getBaseUrl(),config.login_path);
    debug!("Send request to: {}",url);
    let res = client.post(url.as_str())
        .header("tenant",config.tenant_id.to_string())
        .header("secret",config.client_secret.to_string()).send().await?.json::<LoginResponse>().await?;



    Result::Ok(res)
}