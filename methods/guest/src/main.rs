#![no_main]

use ethabi::Token;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub type Memory<const SIZE: usize> = Box<[u8; SIZE]>;

/// Maximum memory in MiB
pub const FUEL_MAX_MEMORY_SIZE: u64 = 1;

/// Maximum VM RAM, in bytes.
pub const VM_MAX_RAM: u64 = 1024 * 1024 * FUEL_MAX_MEMORY_SIZE;

/// Size of the VM memory, in bytes.
#[allow(clippy::cast_possible_truncation)]
pub const MEM_SIZE: usize = VM_MAX_RAM as usize;


pub fn main() {
    
    // let _memory = include_bytes!("blob").to_vec();
    let _memory: Memory<MEM_SIZE> = vec![0; MEM_SIZE]
            .try_into()
            .expect("Failed to allocate memory");

    env::commit_slice(&ethabi::encode(&[Token::FixedBytes(vec![0; 32])]));
}


// #![no_main]

// use ethabi::Token;
// use risc0_zkvm::guest::env;
// use prover_core::{check_transition, Inputs};

// risc0_zkvm::guest::entry!(main);

// pub fn main() {
//     let Inputs {chain_config, target_block, transaction_json} = env::read();

//     let block_id = check_transition(&chain_config, &target_block, &transaction_json);

//     env::commit_slice(&ethabi::encode(&[Token::FixedBytes(block_id.as_slice().to_vec())]));
// }