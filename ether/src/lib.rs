use thiserror::Error;

pub mod etherscan;
pub mod types;
use etherscan::ApiError;
pub use etherscan::EtherscanApi;

#[derive(Debug, Error)]

pub enum ClientError {
    #[error(transparent)]
    ApiError(#[from] ApiError),
}
