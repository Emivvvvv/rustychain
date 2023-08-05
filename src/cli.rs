use clap;

/// Runs the clap app in order to use cli
pub fn cli() -> clap::Command {
    clap::Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            clap::Arg::new("private-key")
                .required_unless_present_any(["node", "dev"])
                .index(1)
                .help("Your private key."),
        )
        .arg(
            clap::Arg::new("miner")
                .conflicts_with("node")
                .required_unless_present_any(["node", "dev"])
                .short('m')
                .long("miner")
                .action(clap::ArgAction::SetTrue)
                .help("Opens rustychain as a miner. To mine blocks use this flag.")
        )
        .arg(
            clap::Arg::new("node")
                .short('n')
                .long("node")
                .action(clap::ArgAction::SetTrue)
                .help("Opens rustychain as a node. You can only watch and save the blockchain data. You can make queries on blockchain.")
        )
        .arg(
            clap::Arg::new("dev")
                .short('d')
                .long("dev")
                .action(clap::ArgAction::SetTrue)
                .help("Dev mode for testing.")
        )
}