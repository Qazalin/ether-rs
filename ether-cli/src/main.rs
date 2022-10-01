mod opts;
use crate::opts::{BalanceArgs, BlockArgs, Subcommands};
use clap::Parser;
use ether::etherscan::{EtherscanApi, EtherscanApiConfig};

#[tokio::main]
async fn main() {
    let mut etherscan_api = EtherscanApi::new(EtherscanApiConfig {
        api_key: None,
        user: None,
    });

    let commands = opts::Commands::parse();

    match commands.sub {
        Subcommands::Balance(BalanceArgs { address, me }) => {
            if me {
                println!("Getting balance for authenticated user");
            } else {
                println!("Getting balance for {}", address);
                let balance = etherscan_api.get_balance(address).await.unwrap();
                println!("Balance: {}", balance);
            }
        }
        Subcommands::Block(BlockArgs {}) => {
            let block_num = etherscan_api.get_block().await.unwrap();
            println!("Block number: {}", block_num);
        }
    }
}
