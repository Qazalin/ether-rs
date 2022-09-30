use ::clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[clap(version = "0.0.1", author = "qazalin", name = "ether")]
pub struct EtherCliCommands {
    /// Get the balance of an Ethereum address
    #[clap(subcommand)]
    pub subcommands: EtherCliSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum EtherCliSubcommands {
    /// Get the balance of an Ethereum address
    #[clap(name = "balance", about = "Get the balance of an Ethereum address")]
    Balance(BalanceAddressArgs),
}

#[derive(Parser, Debug)]
pub struct BalanceAddressArgs {
    /// The address to get the balance of
    #[clap(long, short = 'a', required = false)]
    pub address: String,
    /// The ERC20 token address
    #[clap(long, short = 't', required = false)]
    pub token: String,
    /// Get the balance for the authenticated
    #[clap(long = "me", short = 'm', required = false)]
    pub me: bool,
}
