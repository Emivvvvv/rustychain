use rustychain::cli::{cli, Args, AppMode};
use rustychain::blockchain::Chain;
use rustychain::miner::Miner;

fn main() {
    let cli = cli();
    let args = Args::get_args(cli.get_matches());

    let mut chain = Chain::init();

    match args.get_app_mode() {
        AppMode::Miner(threads) => {
            let mut miner = Miner::new(args.get_private_key(), threads.to_owned());
        }
        AppMode::Node => {

        }
    }
}
