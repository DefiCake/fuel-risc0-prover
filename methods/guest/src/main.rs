#![no_main]

use ethabi::Token;
use risc0_zkvm::guest::env;
use sha2::{Digest, Sha256};
use std::io::Read;


risc0_zkvm::guest::entry!(main);

pub fn main() {
    let mut input = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input).unwrap();

    // let _interpreter: Interpreter<u64, fuel_tx::Script> = Default::default();
    // let _interpreter = prover_core::initialize_interpreter();

    let hash = Sha256::new().chain_update(&input).finalize().to_vec();

    env::commit_slice(&ethabi::encode(&[Token::FixedBytes(hash)]));
}
