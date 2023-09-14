use anyhow::{Context, Result};
use clap::Parser;
use ethabi::ParamType;
use methods::{X509_ELF, X509_ID};
use risc0_zkvm::{Executor, ExecutorEnv};
use std::{
    io::{BufReader, Read},
    path::PathBuf,
};

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

    let env = ExecutorEnv::builder().add_input(&buf).build().unwrap();

    // Next, we make an executor, loading the (renamed) ELF binary.
    let mut exec = Executor::from_elf(env, X509_ELF).context("Failed to instantiate executor")?;
    // let mut exec = default_executor_from_elf(env, X509_ELF).unwrap();

    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(X509_ID).unwrap();

    // We can extract the output of the journal
    let out = ethabi::decode(&[ParamType::FixedBytes(32)], &receipt.journal)?;

    println!(
        "Hash of snapshot {:?}",
        hex::encode(out[0].clone().into_fixed_bytes().unwrap())
    );

    Ok(())
}
