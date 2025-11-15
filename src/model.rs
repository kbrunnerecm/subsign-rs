use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct LoginResponse{
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i32,
    pub refresh_expires: i32
}

impl LoginResponse {
    pub fn new(access_token: &str, refresh_token: &str, expires_in: i32, refresh_expires: i32) -> LoginResponse {
       Self::new(access_token, refresh_token, expires_in, refresh_expires)
    }

    pub fn empty() -> LoginResponse {
        Self::new("", "", 0,0)
    }
}


#[derive(Debug,Deserialize)]
pub struct SubsignConfig {
    pub tenant_id: String,
    pub client_secret: String,
    pub protocol: String,
    pub host: String,
    pub port: i32,
    pub login_path: String
}


impl SubsignConfig {
    pub fn getBaseUrl(&self) -> String {
        format!("{}://{}:{}", self.protocol,self.host, self.port)
    }
}