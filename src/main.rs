use rustychain::cli::{cli, Args, AppMode};
use rustychain::blockchain::Chain;
use rustychain::miner::Miner;
use rustychain::node::Node;

fn main() {
    let cli = cli();
    let args = Args::get_args(cli.get_matches());

    let chain = Chain::init();

    match args.get_app_mode() {
        AppMode::Miner(threads) => {
            let mut miner = Miner::new(chain, args.get_private_key(), threads.to_owned());
            miner.mine();
        }
        AppMode::Node => {
            let node = Node::new(chain);
            node.watch();
        }
    }
}
