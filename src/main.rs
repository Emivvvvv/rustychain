use rustychain::cli::cli;
use rustychain::blockchain::Chain;
use rustychain::sha128::Sha128;

fn main() {
    let cli = cli();
    let matches = cli.get_matches();

    let miner_flag = matches.get_flag("miner");
    let node_flag = matches.get_flag("node");
    let dev_flag = matches.get_flag("dev");
    let private_key_string = match matches.get_one::<String>("private-key") {
        Some(pk) => pk.to_owned(),
        None => String::new()
    };
    let private_key = private_key_string.parse::<u128>().unwrap();

    println!("miner_flag: {miner_flag}, node_flag: {node_flag}, dev_flag: {dev_flag}, private_key: {private_key}");

    let mut chain = Chain::init();
    chain.test(private_key);
    chain.sign_transaction(private_key, Sha128::address(31), 25);
    chain.sign_transaction(private_key, Sha128::address(31), 25);
    println!("curr_trans: {:?},\n\n chain: {:?}\n\n accounts: {:?}\n\n", chain.get_curr_transactions(), chain.get_chain(), chain.get_accounts());

    chain.mine();

    println!("curr_trans: {:?},\n\n chain: {:?}\n\n accounts: {:?}", chain.get_curr_transactions(), chain.get_chain(), chain.get_accounts());
}
