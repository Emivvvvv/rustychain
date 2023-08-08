use clap;
use clap::ArgMatches;

pub struct Args {
    private_key: u128,
    threads: u8,
}

/// Runs the clap app in order to use cli
pub fn cli() -> clap::Command {
    clap::Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            clap::Arg::new("private-key")
                .required(true)
                .index(1)
                .help("Your private key."),
        )
        .arg(
            clap::Arg::new("threads")
                .short('t')
                .long("threads")
                .default_value("8")
                .help("Number of threads to be used."),
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


        let private_key = private_key_string.parse::<u128>().expect("private key must be u128 binary format(for now)");

        Args {
            private_key,
            threads,
        }
    }

    pub fn get_private_key(&self) -> u128 {
        self.private_key
    }

    pub fn get_threads(&self) -> u8 {
        self.threads
    }
}