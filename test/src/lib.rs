mod helpers;

pub use fuel_core::service::ServiceTrait;
pub use prover_core::check_transition;
pub use crate::helpers::{bootstrap1, snapshot, SnapshotStringify};

pub const TEST_SNAPSHOT: &str = include_str!("../res/test_snapshot.json");
pub const TEST_BLOCK: &str = include_str!("../res/test_target_block.json");
pub const TEST_TRANSACTION: &str = include_str!("../res/test_transaction.json");

#[tokio::test]
async fn test_initialize_interpreter() -> anyhow::Result<()> {

    let (srv, _provider) = bootstrap1().await.expect("Could not bootstrap node");

    srv.stop_and_await().await.expect("Could not shutdown node");

    let initial_state = snapshot(&srv)?;
    let _stringified_initial_state = initial_state.stringify()?; // To be used at check_transition(state, _, _)
    // next: import util

    let _block_id = check_transition(TEST_SNAPSHOT, TEST_BLOCK, TEST_TRANSACTION);

    Ok(())
}