use anyhow::{Context, Result};
use clap::Parser;
use ethabi::ParamType;
use methods::{X509_ELF, X509_ID};
use prover_core::Inputs;
use risc0_zkvm::{ExecutorImpl, ExecutorEnv};
use std::{
    io::{BufReader, Read},
    path::PathBuf,
};

const OUTPUT_PARAM_TYPES: [ParamType; 1] = [
  ParamType::FixedBytes(32),
];

#[derive(Parser)]
struct Args {
    path: Option<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    let path = &args
        .path
        .unwrap_or_else(|| PathBuf::from("res/snapshot.json"));
    let file = std::fs::File::open(&path).unwrap();

    let mut buf_reader = BufReader::new(file);
    let mut buf = Vec::new();
    buf_reader
        .read_to_end(&mut buf)
        .expect("Could not read from buffer");

    let inputs = Inputs {
        chain_config: String::from(include_str!("../../core/res/test_snapshot.json")),
        target_block: String::from(include_str!("../../core/res/test_target_block.json")),
        transaction_json: String::from(include_str!("../../core/res/test_transaction.json")),
    };

    let env = 
        ExecutorEnv::builder()
            .write(&inputs)
            .unwrap()
            .build()
            .unwrap();

    // Next, we make an executor, loading the (renamed) ELF binary.
    let mut exec = ExecutorImpl::from_elf(env, X509_ELF).context("Failed to instantiate executor")?;
    // let mut exec = default_executor_from_elf(env, X509_ELF).unwrap();

    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(X509_ID).unwrap();

    // We can extract the output of the journal
    let out = ethabi::decode(
        &OUTPUT_PARAM_TYPES,
        &receipt.journal.bytes
    ).unwrap();

    println!(
        "Hash of snapshot {:?}",
        hex::encode(out[0].clone().into_fixed_bytes().unwrap())
    );

    Ok(())
}
