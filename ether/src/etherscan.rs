use crate::types::User;
use reqwest::{Client, ClientBuilder, Url};

#[derive(Debug)]
pub struct EtherscanApi {
    // pub config: Option<EtherscanApiConfig>,
    pub client: Client,
}

impl EtherscanApi {
    pub fn new(cfg: EtherscanApiConfig) -> Self {
        let mut url = Url::parse("https://api.etherscan.io/api").unwrap();
        if let Some(api_key) = cfg.api_key {
            url.query_pairs_mut().append_pair("apiKey", &api_key);
        }

        println!("url: {}", url);

        return Self {
            // config: Some(cfg),
            client: ClientBuilder::new().build().unwrap(),
        };
    }
}

#[derive(Debug)]
pub struct EtherscanApiConfig {
    pub api_key: Option<String>,
    pub user: Option<User>,
}
