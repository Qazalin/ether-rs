use crate::types::{EthercanGeneralResponse, User};
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

    pub async fn get_balance(&mut self, pub_key: String) -> Result<f64, ApiError> {
        self.url.query_pairs_mut().extend_pairs(&[
            ("module", "account"),
            ("action", "balance"),
            ("address", &pub_key),
            ("tag", "latest"),
        ]);

        let res = self.client.get(self.url.clone()).send().await?;
        self.url.query_pairs_mut().clear();
        let data = res.json::<EthercanGeneralResponse<String>>().await?;
        if data.status != "0" {
            let balance = data.result.parse::<f64>().unwrap() / 1e18;
            return Ok(balance);
        }

        return Err(ApiError::EtherscanApiError(data.result));
    }

    pub async fn get_block(&mut self) -> Result<String, ApiError> {
        let timestamp_now = std::time::SystemTime::now();
        let tv = timestamp_now
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.url.query_pairs_mut().extend_pairs(&[
            ("module", "block"),
            ("action", "getblocknobytime"),
            ("timestamp", &tv.to_string()),
            ("closest", "before"),
        ]);

        let res = self.client.get(self.url.clone()).send().await.unwrap();
        self.url.query_pairs_mut().clear();
        let data = res.json::<EthercanGeneralResponse<String>>().await.unwrap();

        if data.status != "0" {
            return Ok(data.result);
        }

        return Err(ApiError::EtherscanApiError(data.result));
    }
}

#[derive(Debug)]
pub struct EtherscanApiConfig {
    pub api_key: Option<String>,
    pub user: Option<User>,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Request API error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Etherscan API error: {0}")]
    EtherscanApiError(String),
}
