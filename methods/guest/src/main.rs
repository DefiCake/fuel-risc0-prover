#![no_main]

use ethabi::Token;
use risc0_zkvm::guest::env;
use sha2::{Digest, Sha256};
use prover_core::{check_transition, Inputs};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let Inputs {chain_config, target_block, transaction_json} = env::read();

    let _block_id = check_transition(&chain_config, &target_block, &transaction_json);
    
    // let _interpreter: Interpreter<u64, fuel_tx::Script> = Default::default();
    // let _interpreter = prover_core::initialize_interpreter();

    let hash = Sha256::new().chain_update(chain_config.as_bytes()).finalize().to_vec();

    env::commit_slice(&ethabi::encode(&[Token::FixedBytes(hash)]));
}
