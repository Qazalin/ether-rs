use ether::etherscan::{EtherscanApi, EtherscanApiConfig};

fn main() {
    let api = EtherscanApi::new(EtherscanApiConfig {
        api_key: Some("0x".to_string()),
        user: None,
    });
    println!("{:?}", api);
}
