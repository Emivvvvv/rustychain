use crate::sha128::Sha128;
use crate::error::MinerError;

pub struct Miner{
    miner_address: u128,
}

impl Miner {
    pub fn new(private_key: u128) -> Self {
        Miner {
            miner_address: Sha128::address(private_key)
        }
    }

    fn mine_curr_block(&self) -> Result<(), MinerError> {
    }
}