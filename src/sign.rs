use std::fmt::format;
use reqwest::Client;
use crate::model::{LoginResponse, SubsignConfig};

pub async  fn login(config: &SubsignConfig,client: &Client) -> Result<LoginResponse,Box<dyn std::error::Error>> {
println!("Try to login");

    let res = client.post(format!("{}{}",config.getBaseUrl(),config.login_path))
        .header("tenant",config.tenant_id.to_string())
        .header("secret",config.client_secret.to_string()).send()
        .await?.json::<LoginResponse>().await?;



    Result::Ok(res)
}