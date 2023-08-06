use crate::sha128::{OVERFLOW_PROTECTION, Sha128};
use crate::error::{AccountDNE, TransactionError};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Accounts {
    accounts_map: HashMap<u128, Account>
}

#[derive(Debug)]
pub struct Account {
    public_key: Option<u128>,
    amount: u128,
}

#[derive(Clone, Debug)]
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
            let receiver_address = transaction.get_receiver_address();
            let mut receiver_account;
            let mut old_amount = 0;
            match self.accounts_map.get(&receiver_address) {
                Some(account) => {
                    let public_key_option = account.public_key;
                    old_amount = account.get_amount();
                    match public_key_option {
                        Some(public_key) => receiver_account = Account::new_with_public_key(public_key),
                        None => receiver_account = Account::new(),
                    }
                },
                None => receiver_account = Account::new(),
            }
            receiver_account.amount = transaction.amount + old_amount;
            self.accounts_map.insert(receiver_address, receiver_account);
        }
    }

    // for testing purposes
    pub fn test_add_coins(&mut self, private_key: u128) {
        let address = Sha128::address(private_key);
        let mut  account = Account::new_with_private_key(private_key);
        account.amount += 50;
        self.accounts_map.insert(address, account);
    }

    pub fn get_account_amount(&self, address: &u128) -> Result<u128, AccountDNE> {
        match &self.accounts_map.get(address) {
            Some(&ref account) => Ok(account.get_amount()),
            None => Err(AccountDNE)
        }
    }

    pub fn sign_transaction(&mut self, private_key: u128, receiver_address: u128, amount: u128) -> Result<Transaction, TransactionError> {
        let sender_address = Sha128::address(private_key);
        match self.accounts_map.get(&sender_address) {
            Some(account) => {
                if account.get_amount() >= amount {
                    let old_amount = account.get_amount();
                    let mut sender_account = Account::new_with_private_key(private_key);
                    sender_account.amount = old_amount - amount;
                    self.accounts_map.insert(sender_address, sender_account);
                    Ok(Transaction::generate_transaction(private_key, receiver_address, amount))
                }
                else { Err(TransactionError("Insufficient amount")) }
            },
            None => Err(TransactionError("Sender account DNE"))
        }
    }
}

impl Account {
    fn new() -> Self {
        Account {
            public_key: None,
            amount: 0,
        }
    }

    fn new_with_private_key(private_key: u128) -> Self {
        Account {
            public_key: Some(Sha128::public_key(private_key)),
            amount: 0
        }
    }

    fn new_with_public_key(public_key: u128) -> Self {
        Account {
            public_key: Some(public_key),
            amount: 0
        }
    }

    fn get_amount(&self) -> u128 {
        self.amount
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

    pub fn transaction_to_miner(receiver_address: u128) -> Self {
        let amount = 6931;
        let system_private_key = 31;
        Transaction {
            hash: Sha128::transaction_hash(Sha128::public_key(system_private_key), receiver_address, amount),
            sender_address: Sha128::address(system_private_key),
            receiver_address,
            amount,
        }
    }

    pub(crate) fn get_receiver_address(&self) -> u128 {
        self.receiver_address
    }

    pub fn get_hash(&self) -> u128 {
        self.hash
    }

    pub(crate) fn get_merkle(curr_trans: &Vec<Transaction>) -> u128 {
        let mut merkle = curr_trans.iter().map(|t| t.get_hash()).collect::<Vec<u128>>();

        if merkle.len() % 2 == 1 {
            merkle.push(merkle.last().cloned().unwrap());
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0) % OVERFLOW_PROTECTION;
            let h2 = merkle.remove(0) % OVERFLOW_PROTECTION;
            h1 = h1 + h2;
            let nh = Sha128::hash(h1);
            merkle.push(nh);
        }
        match merkle.pop() {
            Some(merkle) => merkle,
            None => 0
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "\
               ------------\
               transaction:{}\n\
               from address <{}> -> to address <{}>\n\
               amount:<{}>\
               ------------",
               self.hash,
               self.sender_address,
               self.receiver_address,
               self.amount)
    }
}
