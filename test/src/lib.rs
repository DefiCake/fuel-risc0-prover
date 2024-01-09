mod helpers;

#[cfg(test)]
mod tests {
    use fuel_core::service::ServiceTrait;
    use prover_core::check_transition;
    use crate::helpers::bootstrap1;

    const TEST_SNAPSHOT: &str = include_str!("../res/test_snapshot.json");
    const TEST_BLOCK: &str = include_str!("../res/test_target_block.json");
    const TEST_TRANSACTION: &str = include_str!("../res/test_transaction.json");
    #[tokio::test]
    async fn test_initialize_interpreter() {

        let srv = bootstrap1().await.expect("Could not bootstrap node");

        let _block_id = check_transition(TEST_SNAPSHOT, TEST_BLOCK, TEST_TRANSACTION);

        let _state = srv.stop_and_await().await.expect("Could not shutdown node");
    }
}