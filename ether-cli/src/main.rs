mod opts;

use crate::opts::EtherCliCommands;
use clap::Parser;
use ether::etherscan::{EtherscanApi, EtherscanApiConfig};

fn main() {
    let _api = EtherscanApi::new(EtherscanApiConfig {
        api_key: Some("0x".to_string()),
        user: None,
    });

    let commands = EtherCliCommands::parse();

    // println!("{:?}", api);
    println!("{:?}", commands);
}
