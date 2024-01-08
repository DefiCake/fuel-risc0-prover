use anyhow::Result;

fn main() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use prover_core::check_transition;

    const TEST_SNAPSHOT: &str = include_str!("../res/test_snapshot.json");
    const TEST_BLOCK: &str = include_str!("../res/test_target_block.json");
    const TEST_TRANSACTION: &str = include_str!("../res/test_transaction.json");
    #[test]
    fn test_initialize_interpreter() {
        let _block_id = check_transition(TEST_SNAPSHOT, TEST_BLOCK, TEST_TRANSACTION);
    }
}