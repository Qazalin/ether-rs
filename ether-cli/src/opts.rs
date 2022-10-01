use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(version = "0.0.1", author = "qazalin", name = "ether")]
pub struct Commands {
    /// The action to perform
    #[clap(subcommand)]
    pub sub: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    /// Get the balance for an Ethereum address
    #[clap(name = "balance")]
    Balance(BalanceArgs),
    /// Get blockchain info
    #[clap(name = "block")]
    Block(BlockArgs),
}

#[derive(Parser, Debug)]
pub struct BalanceArgs {
    /// The balance of the Ethereum address
    #[clap(short = 'a', long = "address")]
    pub address: String,
    /// Get the balance for the authenticated users
    #[clap(short = 'm', long = "me")]
    pub me: bool,
}

#[derive(Parser, Debug)]
pub struct BlockArgs {}
