use crate::accounts::{Accounts, Transaction};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::sha128::{Sha128, OVERFLOW_PROTECTION};

pub struct Chain {
    chain: Vec<Block>,
    accounts: Accounts,
    curr_trans: Vec<Transaction>,
    length: usize,
    difficulty: u8,
    reward: u8,
}

impl Chain {
    pub fn init() -> Self {
        let difficulty: u8 = 20;
        let chain: Vec<Block> = vec![Block::get_genesis_block(difficulty)];
        let accounts = Accounts::new();
        let curr_trans: Vec<Transaction> = Vec::new();
        let length: usize = 1;
        let reward: u8 = 50;

        Chain {
            chain,
            accounts,
            curr_trans,
            length,
            difficulty,
            reward,
        }
    }

    pub fn test(&mut self, private_key: u128) {
        self.accounts.test_add_coins(private_key)
    }

    pub fn mine(&mut self) {
        let last_block = &self.chain[self.length - 1];
        let curr_trans = self.curr_trans.clone();
        self.curr_trans = Vec::new();

        let mut new_block = Block::new_block(last_block, curr_trans.clone());
        Chain::pow(&mut new_block);
        new_block.header.time = get_time();
        self.accounts.update_accounts(curr_trans);

        self.chain.push(new_block)
    }


    pub fn pow(block: &mut Block) {
        while Sha128::block_hash(&block.header) > std::u128::MAX >> block.get_difficulty(){
            block.header.nonce += 1
        }
    }

    pub fn sign_transaction(&mut self, private_key: u128, receiver_address: u128, amount: u128) {
        match &self.accounts.sign_transaction(private_key, receiver_address, amount) {
            Ok(transaction) => self.curr_trans.push(transaction.clone()),
            Err(err) => panic!("{}", err),
        }
    }

    pub fn get_curr_transactions(&self) -> &Vec<Transaction> {
        &self.curr_trans
    }

    pub fn get_chain(&self) -> &Vec<Block> {
        &self.chain
    }

    pub fn get_accounts(&self) -> &Accounts {
        &self.accounts
    }
}

fn get_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros()
}

#[derive(Debug)]
pub struct BlockHeader {
    time: u128,
    nonce: u128,
    pre_hash: u128,
    merkle: u128,
    difficulty: u8,
}

impl BlockHeader {
    pub fn get_data(&self) -> u128 {
        self.nonce % OVERFLOW_PROTECTION +
            self.pre_hash % OVERFLOW_PROTECTION +
            self.merkle % OVERFLOW_PROTECTION +
            self.difficulty as u128
    }
}

#[derive(Debug)]
pub struct Block {
    header: BlockHeader,
    transactions: Vec<Transaction>
}

impl Block {
     fn get_genesis_block(difficulty: u8) -> Self {
        let time = get_time();
        let pre_hash: u128 = 0;
        let nonce: u128 = 0;
        let merkle: u128 = 0;
        let header = BlockHeader {
            time,
            nonce,
            pre_hash,
            merkle,
            difficulty
        };

        let transactions: Vec<Transaction> = Vec::new();

        Block {
            header,
            transactions
        }
    }

    fn new_block(prev_block: &Block, transactions: Vec<Transaction>) -> Block {
        Block {
            header: BlockHeader {
                time: 0,
                nonce: 0,
                pre_hash: prev_block.get_hash(),
                merkle: Transaction::get_merkle(&transactions),
                difficulty: prev_block.get_difficulty(),
            },
            transactions,
        }
    }

    fn get_hash(&self) -> u128 {
        Sha128::block_hash(&self.header)
    }

    fn get_difficulty(&self) -> u8 {
        self.header.difficulty
    }
}

