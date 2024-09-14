use std::fmt;
use crate::accounts::{Accounts, Transaction};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::error::ChainError;
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
        let difficulty: u8 = 24;
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

    pub fn check_health(&self, chain: &Chain) -> Result<(), ChainError> {
        let curr_block_hash = self.chain[self.length - 1].get_hash();
        let new_chain_prev_has = chain.chain[chain.length - 1].header.pre_hash;

        if curr_block_hash == new_chain_prev_has {
            if self.chain[..self.length - 1] == chain.chain[..self.length - 2] {
                Ok(())
            } else {
                Err(ChainError("Bad chain! Chain is broken!"))
            }
        }
        else { Err(ChainError("Bad block! Prev has is not correct!")) }
    }

    pub fn print_all_blockchain(&self) {
        for block in &self.chain {
            println!("{block}")
        }
    }

    pub fn print_current(&self) {
        println!();
        println!("Last block:\n{}", &self.chain[self.length - 1]);
        println!();

        let transactions = &self.curr_trans;

        if transactions.is_empty() {println!("No new transactions.")}
        else {println!("Current transactions:");}

        for transaction in transactions {
            println!("{transaction}")
        }

    }

    pub fn mine(&mut self, miner_address: u128, num_threads: u8) {
        self.miner_reward_transaction(miner_address, self.reward);

        let last_block = &self.chain[self.length - 1];
        let curr_trans = self.curr_trans.clone();
        self.curr_trans = Vec::new();

        let mut new_block = Block::new_block(last_block, curr_trans.clone());
        self.pow(&mut new_block, num_threads);

        new_block.header.time = get_time();
        self.accounts.update_accounts(curr_trans);
        self.length += 1;

        self.chain.push(new_block)
    }

    pub fn pow(&self, block: &mut Block, num_threads: u8) {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::thread;
        use std::sync::mpsc;

        let difficulty = self.difficulty as u128;

        let (tx, rx) = mpsc::channel();

        let found = Arc::new(AtomicBool::new(false));
        let block_header = Arc::new(block.header.clone());

        let mut handles = Vec::new();
        for i in 0..num_threads {
            let tx = tx.clone();
            let found = Arc::clone(&found);
            let block_header = Arc::clone(&block_header);
            let thread_id = i;

            let handle = thread::spawn(move || {
                let mut local_header = (*block_header).clone();
                local_header.nonce = thread_id as u128;

                while !found.load(Ordering::Relaxed) {
                    let hash = Sha128::block_hash(&local_header);
                    if hash >> (128 - difficulty) == 0 {
                        if !found.swap(true, Ordering::Relaxed) {
                            let _ = tx.send(local_header.nonce);
                        }
                        break;
                    } else {
                        local_header.nonce += num_threads as u128;
                    }
                }
            });

            handles.push(handle);
        }

        drop(tx);

        if let Ok(nonce) = rx.recv() {
            block.header.nonce = nonce;
        }

        for handle in handles {
            let _ = handle.join();
        }
    }

    pub fn miner_reward_transaction(&mut self, miner_address: u128, reward: u8) {
        self.sign_transaction(31, miner_address, reward as u128);
    }
    pub fn sign_transaction(&mut self, private_key: u128, receiver_address: u128, amount: u128) {
        match &self.accounts.sign_transaction(private_key, receiver_address, amount) {
            Ok(transaction) => {
                println!("New Transaction:\n{}", transaction);
                self.curr_trans.push(transaction.clone())
            },
            Err(err) => println!("{}", err),
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

#[derive(PartialEq, Clone)]
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
#[derive(PartialEq)]
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

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "| previous block's hash: {:032x}\n\
               | mining time: {}\n\
               | nonce: {}\n\
               | merkle: {:032x}\n\
               | difficulty: {}\n\
               | block hash: {:032x}",
               self.header.pre_hash,
               self.header.time,
               self.header.nonce,
               self.header.merkle,
               self.header.difficulty,
               self.get_hash(),
               )
    }
}

