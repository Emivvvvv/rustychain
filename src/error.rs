use thiserror::Error;

#[derive(Error, Debug)]
#[error("Account does not exists!")]
pub struct AccountDNE;

#[derive(Error, Debug)]
#[error("Transaction Error: {0}")]
pub struct TransactionError(pub String);

#[derive(Error, Debug)]
#[error("Miner Error: {0}")]
pub struct MinerError(pub &'static str);

#[derive(Error, Debug)]
#[error("Chain Error: {0}")]
pub struct ChainError(pub &'static str);