use std::time::Duration;
use rustychain::cli::{cli, Args};
use rustychain::miner::Miner;

const COMMAND_STRING: &str = "Type command\n\
            :m => to mine\n\
            :p => to print last block and the current transactions\n\
            :pa => to print all accounts and their balances\n\
            :s => to send a transaction from the miners private key\n\
            :ss => to send a transaction with a new private key\n\
            :q => to quit\n";

const INCORRECT_COMMAND_STRING: &str = "Please check menu and type one of the correct commands. <ex type ':m'>";

fn string_input() -> String {
    std::thread::sleep(Duration::from_millis(1000));
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("ailed to read from stdin");
    input.trim().to_string()
}

fn string_to_u128(string: String) -> u128 {
    match string.parse::<u128>() {
        Ok(i) => i,
        Err(_) => panic!("Input is not an integer! Please try again\n"),
    }
}

fn main() {
    let cli = cli();
    let args = Args::get_args(cli.get_matches());

    let mut miner = Miner::new(args.get_private_key(), args.get_threads());
    println!("{}",COMMAND_STRING);

    let mut command;

    loop {
        command = string_input();
        match command.as_str() {
            ":m" | "m" => miner.mine(),
            ":p" | "p" => miner.print_current(),
            ":pa" | "pa" => miner.print_accounts(),
            ":s" | "s" => {
                println!("Enter receiver address");
                let receiver_address = string_to_u128(string_input());
                println!("Enter amount");
                let amount = string_to_u128(string_input());
                miner.send_transaction(receiver_address, amount);
            },
            ":ss" | "ss" => {
                println!("Enter sender private key");
                let sender_private_key = string_to_u128(string_input());
                println!("Enter receiver address");
                let receiver_address = string_to_u128(string_input());
                println!("Enter amount");
                let amount = string_to_u128(string_input());
                miner.send_transaction_with_another_private_key(sender_private_key, receiver_address, amount);
            },
            ":q" | "q" => break,
            _ => {
                println!("{}", INCORRECT_COMMAND_STRING);
                command = string_input()
            },
        }
    }
}
