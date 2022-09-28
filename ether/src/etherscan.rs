use crate::types::User;
use reqwest::{Client, ClientBuilder, Url};

#[derive(Debug)]
pub struct EtherscanApi {
    // pub config: Option<EtherscanApiConfig>,
    pub client: Client,
}

impl EtherscanApi {
    pub fn new(cfg: EtherscanApiConfig) -> Self {
        let mut builder = ClientBuilder::new();
        if let Some(api_key) = cfg.api_key {
            let mut url =
                Url::parse_with_params("https://api.etherscan.io/api", &[("apikey", api_key)])
                    .unwrap();
            builder = builder.user_agent("ether");
        }
        let client = builder.build().unwrap();

        EtherscanApi {
            client: Client::new(),
        }
    }
}

#[derive(Debug)]
pub struct EtherscanApiConfig {
    pub api_key: Option<String>,
    pub user: Option<User>,
}
