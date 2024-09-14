use crate::sha128::{OVERFLOW_PROTECTION, Sha128};
use crate::error::{AccountDNE, TransactionError};
use std::collections::HashMap;
use std::fmt;

#[derive(Default)]
pub struct Accounts {
    accounts_map: HashMap<u128, Account>
}

pub struct Account {
    #[allow(dead_code)]
    public_key: Option<u128>,
    amount: u128,
    pending_amount: u128,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Transaction {
    hash: u128,
    sender_address: u128,
    receiver_address: u128,
    amount: u128,
}

impl Accounts {
    pub fn new() -> Self {
        let mut accounts_map = HashMap::new();
        let mut system_account = Account::new_with_private_key(31);
        system_account.amount = 21_000_000;
        accounts_map.insert(Sha128::address(31), system_account);
        Accounts {
            accounts_map,
        }
    }

    pub(super) fn update_accounts(&mut self, transactions: Vec<Transaction>) {
        for transaction in transactions {
            let sender_address = transaction.get_sender_address();
            if let Some(sender_account) = self.accounts_map.get_mut(&sender_address) {
                sender_account.amount -= transaction.amount;
                sender_account.pending_amount -= transaction.amount;
            };

            let receiver_address = transaction.get_receiver_address();
            if let Some(receiver_account) = self.accounts_map.get_mut(&receiver_address) {
                receiver_account.amount += transaction.amount;
            } else {
                let mut new_account = Account::new();
                new_account.amount += transaction.amount;
                self.accounts_map.insert(receiver_address, new_account);
            }
        }
    }

    pub fn get_account_amount(&self, address: &u128) -> Result<u128, AccountDNE> {
        match &self.accounts_map.get(address) {
            Some(account) => Ok(account.get_amount()),
            None => Err(AccountDNE)
        }
    }

    pub fn sign_transaction(&mut self, private_key: u128, receiver_address: u128, amount: u128) -> Result<Transaction, TransactionError> {
        let sender_address = Sha128::address(private_key);
        match self.accounts_map.get_mut(&sender_address) {
            Some(account) => {
                if account.get_amount() - account.get_pending_amount() >= amount {
                    account.pending_amount += amount;
                    Ok(Transaction::generate_transaction(private_key, receiver_address, amount))
                }
                else {
                    let err_str = format!("Insufficient amount! amount: `{}`, pending amount: `{}`", account.get_amount(), account.get_pending_amount());
                    Err(TransactionError(err_str))
                }
            },
            None => Err(TransactionError("Sender account DNE".to_string()))
        }
    }

    pub fn print_accounts(&self) {
        for (account_address, account) in &self.accounts_map {
            println!("account address: {}, account balance: {}", account_address, account.get_amount())
        }
    }
}

impl Account {
    fn new() -> Self {
        Account {
            public_key: None,
            amount: 0,
            pending_amount: 0,
        }
    }

    fn new_with_private_key(private_key: u128) -> Self {
        Account {
            public_key: Some(Sha128::public_key(private_key)),
            amount: 0,
            pending_amount: 0,
        }
    }

    fn get_amount(&self) -> u128 {
        self.amount
    }

    fn get_pending_amount(&self) -> u128 {
        self.pending_amount
    }
}

impl Transaction {
    fn generate_transaction(private_key: u128, receiver_address: u128, amount: u128) -> Self {
        Transaction {
            hash: Sha128::transaction_hash(Sha128::public_key(private_key), receiver_address, amount),
            sender_address: Sha128::address(private_key),
            receiver_address,
            amount,
        }
    }

    pub fn transaction_to_miner(receiver_address: u128, amount: u8) -> Self {
        let amount = amount as u128;
        let system_private_key = 31;
        let miner_reward_transaction = Transaction {
            hash: Sha128::transaction_hash(Sha128::address(system_private_key), receiver_address, amount),
            sender_address: Sha128::address(system_private_key),
            receiver_address,
            amount,
        };

        println!("Miner reward transaction\n{}\n", miner_reward_transaction);

        miner_reward_transaction
    }

    pub(crate) fn get_sender_address(&self) -> u128 {
        self.sender_address
    }

    pub(crate) fn get_receiver_address(&self) -> u128 {
        self.receiver_address
    }

    pub fn get_hash(&self) -> u128 {
        self.hash
    }

    pub(crate) fn get_merkle(curr_trans: &[Transaction]) -> u128 {
        let mut merkle = curr_trans.iter().map(|t| t.get_hash()).collect::<Vec<u128>>();

        if merkle.len() % 2 == 1 {
            merkle.push(merkle.last().cloned().unwrap());
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0) % OVERFLOW_PROTECTION;
            let h2 = merkle.remove(0) % OVERFLOW_PROTECTION;
            h1 += h2;
            let nh = Sha128::hash(h1);
            merkle.push(nh);
        }
        merkle.pop().unwrap_or(0)
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "\
               ------------\n\
               transaction:{}\n\
               from address <{}> -> to address <{}>\n\
               amount:<{}>\n\
               ------------",
               self.hash,
               self.sender_address,
               self.receiver_address,
               self.amount)
    }
}
