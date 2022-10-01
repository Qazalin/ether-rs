use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(version = "0.0.1", author = "qazalin", name = "ether")]
pub struct EtherCliCommands {
    /// The action to perform
    #[clap(subcommand)]
    pub subcommand: EtherCliSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum EtherCliSubcommands {
    /// Get the balance for an Ethereum address
    #[clap(name = "balance")]
    Balance(EtherBalanceArgs),
    /// Get blockchain info
    #[clap(name = "block")]
    Block(EtherBlockArgs),
}

#[derive(Parser, Debug)]
pub struct EtherBalanceArgs {
    /// The balance of the Ethereum address
    #[clap(short = 'a', long = "address")]
    pub address: String,
    /// The ERC20 token address
    #[clap(short = 't', long = "token")]
    pub token: String,
    /// Get the balance for the authenticated users
    #[clap(short = 'm', long = "me")]
    pub me: bool,
}

#[derive(Parser, Debug)]
pub struct EtherBlockArgs {}
