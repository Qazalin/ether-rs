mod opts;

// use crate::opts::EtherCliCommands;
// use clap::Parser;
use ether::etherscan::{EtherscanApi, EtherscanApiConfig};

#[tokio::main]
async fn main() {
    let mut api = EtherscanApi::new(EtherscanApiConfig {
        api_key: None,
        user: None,
    });

    let d = api
        .get_balance("0x7b9e0e62e3798ffb33326421a7d203ffd60b711c".to_string())
        .await
        .ok();

    println!("{:?}", d);
}
