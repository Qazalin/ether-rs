use thiserror::Error;

pub mod etherscan;
pub mod types;
pub use etherscan::EtherscanApi;
use etherscan::EtherscanApiError;

#[derive(Debug, Error)]

pub enum ClientError {
    #[error(transparent)]
    OpenSeaApiError(#[from] EtherscanApiError),
}
