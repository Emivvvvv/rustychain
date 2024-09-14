use sha2::{Sha256, Digest};
use crate::blockchain::BlockHeader;

pub const OVERFLOW_PROTECTION: u128 = u128::MAX >> 1;

fn u128_to_u8_array(n: u128) -> [u8; 16] {
    let mut result = [0u8; 16];

    for (i, byte) in result.iter_mut().enumerate() {
        *byte = ((n >> (8 * (15 - i))) & 0xFF) as u8;
    }

    result
}

fn u8_array_to_u128(array: [u8; 16]) -> u128 {
    let mut result: u128 = 0;

    for (i, &byte) in array.iter().enumerate() {
        result |= u128::from(byte) << (8 * (15 - i));
    }

    result
}


fn sha128(input: u128) -> u128 {
    let mut hasher = Sha256::new();
    hasher.update(u128_to_u8_array(input));
    let hash = hasher.finalize();
    u8_array_to_u128(hash[..16].try_into().unwrap())
}

pub struct Sha128;

impl Sha128 {
    pub fn hash(input: u128) -> u128 {
        sha128(input)
    }

    pub fn public_key(private_key: u128) -> u128 {
        sha128(private_key)
    }

    pub fn address(private_key: u128) -> u128 {
        sha128(Sha128::public_key(private_key))
    }

    pub fn transaction_hash(public_key: u128, receiver_address: u128, amount: u128) -> u128 {
        sha128(
            public_key % OVERFLOW_PROTECTION +
            receiver_address % OVERFLOW_PROTECTION +
            amount % OVERFLOW_PROTECTION)
    }

    pub fn block_hash(header: &BlockHeader) -> u128 {
        sha128(header.get_data())
    }
}