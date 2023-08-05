use clap;
use clap::ArgMatches;

pub struct Args {
    private_key: u128,
    app_mode: AppMode
}

pub enum AppMode {
    Miner(u8),
    Node,
}

/// Runs the clap app in order to use cli
pub fn cli() -> clap::Command {
    clap::Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            clap::Arg::new("private-key")
                .required_unless_present("node")
                .index(1)
                .help("Your private key."),
        )
        .arg(
            clap::Arg::new("miner")
                .conflicts_with("node")
                .required_unless_present("node")
                .short('m')
                .long("miner")
                .action(clap::ArgAction::SetTrue)
                .help("Opens rustychain as a miner. To mine blocks use this flag. You can send transactions while mining.")
        )
        .arg(
            clap::Arg::new("threads")
                .conflicts_with("node")
                .short('t')
                .long("threads")
                .default_value("8")
                .help("Number of threads to be used."),
        )
        .arg(
            clap::Arg::new("node")
                .short('n')
                .long("node")
                .action(clap::ArgAction::SetTrue)
                .help("Opens rustychain as a node. You can only watch and send transactions to the network.")
        )
}

impl Args {
    pub fn get_args(matches: ArgMatches) -> Self{
        let private_key_string = match matches.get_one::<String>("private-key") {
            Some(pk) => pk.to_owned(),
            None => String::new()
        };

        let threads = matches.get_one::<String>("threads")
            .expect("This was unexpected :(. Something went wrong while getting -t or --threads arg")
            .trim().parse::<u8>()
            .expect("Threads must be a number!");

        let app_mode = if matches.get_flag("miner") {AppMode::Miner(threads)}
        else {AppMode::Node};

        Args {
            private_key: private_key_string.parse::<u128>().expect("private key must be u128 binary format(for now)"),
            app_mode
        }
    }

    pub fn get_private_key(&self) -> u128 {
        self.private_key
    }

    pub fn get_app_mode(&self) -> &AppMode {
        &self.app_mode
    }
}