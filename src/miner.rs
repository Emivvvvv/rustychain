use crate::blockchain::Chain;
use crate::sha128::Sha128;

pub struct Miner {
    chain: Chain,
    private_key: u128,
    threads: u8,
}

impl Miner {
    pub fn new(private_key: u128, threads: u8) -> Self {
        Miner {
            chain: Chain::init(),
            private_key,
            threads,
        }
    }

    pub fn mine(&mut self) {
        self.chain.mine(Sha128::address(self.private_key), self.threads);
        println!("Block mined successfully");
        Miner::print_current(self);
    }

    pub fn send_transaction(&mut self, receiver_address: u128, amount: u128) {
        self.chain.sign_transaction(self.private_key, receiver_address, amount);
    }

    pub fn send_transaction_with_another_private_key(&mut self, sender_private_key: u128, receiver_address: u128, amount: u128) {
        self.chain.sign_transaction(sender_private_key, receiver_address, amount);
    }

    pub fn print_current(&self) {
        self.chain.print_current();
    }

    pub fn print_accounts(&self) {
        self.chain.get_accounts().print_accounts();
    }
}