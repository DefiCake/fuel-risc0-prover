mod helpers;

pub use fuel_core::service::ServiceTrait;
pub use prover_core::check_transition;

pub use crate::helpers::{bootstrap1, snapshot, SnapshotStringify, block_stringify, alice_sends_bob_100};

pub const TEST_SNAPSHOT: &str = include_str!("../res/test_snapshot.json");
pub const TEST_BLOCK: &str = include_str!("../res/test_target_block.json");
pub const TEST_TRANSACTION: &str = include_str!("../res/test_transaction.json");

#[tokio::test]
async fn test_one_transaction() -> anyhow::Result<()> {

    let (srv, provider) = bootstrap1().await.expect("Could not bootstrap node");
    let initial_state = snapshot(&srv)?;

    
    let _stringified_initial_state = initial_state.stringify()?; // To be used at check_transition(state, _, _)
    // next: import util
    
    
    alice_sends_bob_100(&provider, None, None, true).await?;

    
    let block = srv.shared.database.get_current_block()?.unwrap();
    let _stringified_block = block_stringify(&block)?; // To be used at check_transition(_, block, _)

    // Now, lets get the transactions of a block
    // TODO: get transactions of the target block
    
    // let _block_id = check_transition(TEST_SNAPSHOT, TEST_BLOCK, TEST_TRANSACTION);
    
    srv.stop_and_await().await.expect("Could not shutdown node");
    Ok(())
}