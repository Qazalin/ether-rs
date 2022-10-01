use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct User {
    pub pub_key: String,
    pub sig: Option<String>,
    pub ens: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EthercanGeneralResponse<T> {
    pub status: String,
    pub message: String,
    pub result: T,
}
