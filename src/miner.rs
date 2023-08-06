use crate::sha128::Sha128;
use crate::blockchain::Chain;

pub struct Miner{
    chain: Chain,
    miner_address: u128,
    threads: u8,
}

impl Miner {
    pub fn new(chain: Chain, private_key: u128, threads: u8) -> Self {
        Miner {
            chain,
            miner_address: Sha128::address(private_key),
            threads
        }
    }

    pub fn mine(&mut self) {
        loop {
            &self.chain.print_current();
            self.chain.mine(self.miner_address);
        }
    }
}