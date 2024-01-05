#![no_main]

use ethabi::Token;
use risc0_zkvm::guest::env;
use prover_core::{check_transition, Inputs};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let Inputs {chain_config, target_block, transaction_json} = env::read();

    let block_id = check_transition(&chain_config, &target_block, &transaction_json);

    env::commit_slice(&ethabi::encode(&[Token::FixedBytes(block_id.as_slice().to_vec())]));
}
