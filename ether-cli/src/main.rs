mod opts;
use crate::opts::{BalanceArgs, BlockArgs, Subcommands};
use clap::Parser;
use ether::etherscan::{EtherscanApi, EtherscanApiConfig};
use loading::Loading;

#[tokio::main]
async fn main() {
    let loading = Loading::default();
    let mut etherscan_api = EtherscanApi::new(EtherscanApiConfig {
        api_key: None,
        user: None,
    });

    let commands = opts::Commands::parse();

    match commands.sub {
        Subcommands::Balance(BalanceArgs { address, me }) => {
            if me {
                loading.text("Getting balance for authenticated user");
            } else {
                loading.text(format!("Getting balance for {}", address));
                let balance = etherscan_api.get_balance(address).await.unwrap();
                loading.success(format!("Balance: {} ETH ⬨", balance));
            }
        }
        Subcommands::Block(BlockArgs {}) => {
            loading.text("Getting the latest block number");
            let block_num = etherscan_api.get_block().await.unwrap();
            loading.success(format!("Block number: {}", block_num));
        }
    }
}
