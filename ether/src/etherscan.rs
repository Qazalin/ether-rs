use crate::types::User;
use reqwest::{Client, ClientBuilder, Url};
use thiserror::Error;

#[derive(Debug)]
pub struct EtherscanApi {
    // pub config: Option<EtherscanApiConfig>,
    pub client: Client,
    pub url: Url,
}

impl EtherscanApi {
    pub fn new(cfg: EtherscanApiConfig) -> Self {
        let mut url = Url::parse("https://api.etherscan.io/api").unwrap();
        if let Some(api_key) = cfg.api_key {
            url.query_pairs_mut().append_pair("apiKey", &api_key);
        }

        return Self {
            // config: Some(cfg),
            client: ClientBuilder::new().build().unwrap(),
            url,
        };
    }

    pub async fn get_balance(&mut self, pub_key: String) -> Result<i64, EtherscanApiError> {
        self.url.query_pairs_mut().extend_pairs(&[
            ("module", "account"),
            ("action", "balance"),
            ("address", &pub_key),
            ("tag", "latest"),
        ]);
        let resp = self.client.get(self.url.clone()).send().await?;
        self.url.query_pairs_mut().clear();
        println!("{:?}", resp);
        Ok(0)
    }
}

#[derive(Debug)]
pub struct EtherscanApiConfig {
    pub api_key: Option<String>,
    pub user: Option<User>,
}

#[derive(Debug, Error)]
pub enum EtherscanApiError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}
