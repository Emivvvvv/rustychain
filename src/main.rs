use std::io::{self, Write};

use rustychain::cli::{cli, Args};
use rustychain::miner::Miner;
use rustychain::sha128::Sha128;

const COMMAND_STRING: &str = "Type command\n\
            :m => to mine\n\
            :p => to print last block and the current transactions\n\
            :pa => to print all accounts and their balances\n\
            :s => to send a transaction from the miners private key\n\
            :ss => to send a transaction with a new private key\n\
            :pta => see the address of given private key [for testing]\n\
            :q => to quit\n";

const INCORRECT_COMMAND_STRING: &str = "Please check menu and type one of the correct commands. <ex type ':h' to see commands>";

fn string_input() -> String {
    print!("> "); // print the prompt
    io::stdout().flush().expect("failed to flush stdout"); // manually flush stdout
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    input.trim().to_string() // return the trimmed input
}


fn u128_input() -> Option<u128> {
    loop {
        let input = string_input();
        if input == ":q" || input == "q" {
            return None
        }

        match input.parse::<u128>() {
            Ok(i) => return Some(i),
            Err(_) => println!("Invalid input. Please enter a valid integer. type :q to quit"),
        }
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
                let receiver_address = u128_input();
                println!("Enter amount");
                let amount = u128_input();
                match (receiver_address, amount) {
                    (Some(r_addr), Some(a)) => miner.send_transaction(r_addr, a),
                    _ => println!("Aborting."),
                }
            },
            ":ss" | "ss" => {
                println!("Enter sender private key");
                let sender_private_key = u128_input();
                println!("Enter receiver address");
                let receiver_address = u128_input();
                println!("Enter amount");
                let amount = u128_input();
                match (sender_private_key, receiver_address, amount) {
                    (Some(sender_pk), Some(r_addr), Some(a)) => miner.send_transaction_with_another_private_key(sender_pk, r_addr, a),
                    _ => println!("Aborting."),
                }
            },
            ":q" | "q" => break,
            ":pta" | "pta" => {
                println!("Enter sender private key");
                let private_key = u128_input();
                match private_key {
                    Some(pk) => println!("Public key: {}\nPay Address: {}", Sha128::public_key(pk), Sha128::address(pk)),
                    _ => println!("Aborting."),
                }
            }
            ":h" | "h" => println!("{}", COMMAND_STRING),
            _ => println!("{}", INCORRECT_COMMAND_STRING),
        }
    }
}
