#![no_main]

use ethabi::Token;
use risc0_zkvm::guest::env;
use sha2::{Digest, Sha256};
use std::io::Read;
risc0_zkvm::guest::entry!(main);
use fuel_crypto::SecretKey;
pub fn main() {
    let mut input = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input).unwrap();

    let hash = Sha256::new().chain_update(&input).finalize().to_vec();
    let secret_key = SecretKey::default();
    env::commit_slice(&ethabi::encode(&[Token::FixedBytes(hash)]));
}
