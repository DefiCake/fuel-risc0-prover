mod helpers;

pub use std::ops::Deref;

pub use fuel_core::service::ServiceTrait;
use fuel_core::types::blockchain::block::Block;
use fuels::tx::Bytes32;
pub use prover_core::check_transition;


pub use crate::helpers::{
    bootstrap1, 
    snapshot, 
    SnapshotStringify, 
    block_stringify, 
    txs_stringify, 
    send_funds, 
    get_wallet_by_name, 
    AccountName
};

pub const TEST_SNAPSHOT: &str = include_str!("../res/test_snapshot.json");
pub const TEST_BLOCK: &str = include_str!("../res/test_target_block.json");
pub const TEST_TRANSACTION: &str = include_str!("../res/test_transaction.json");

#[tokio::test]
async fn test_one_transaction_with_artifacts() -> anyhow::Result<()> {
    let block: Block<Bytes32> = 
        serde_json::from_str(TEST_BLOCK)
        .expect("Could not parse target Block");

    let block_id = check_transition(
        TEST_SNAPSHOT,
        TEST_BLOCK,
        TEST_TRANSACTION
    );

    assert_eq!(block.id(), block_id);

    Ok(())
}

#[tokio::test]
async fn test_two_transfers() -> anyhow::Result<()> {

    let (srv, provider) = bootstrap1().await.expect("Could not bootstrap node");
    let initial_state = snapshot(&srv)?;

    
    let stringified_initial_state = initial_state.stringify()?; // To be used at check_transition(state, _, _)
    // next: import util
    
    
    send_funds(&provider, None, None, false).await?;
    send_funds(
        &provider, 
        Some(get_wallet_by_name(AccountName::Carol)), 
        Some(get_wallet_by_name(AccountName::Dave)), 
        true
    ).await?;


    let block = srv.shared.database.get_current_block()?.unwrap();
    let stringified_block = block_stringify(&block)?; // To be used at check_transition(_, block, _)
    
    let block_height = block.header().height().deref().clone();
    let transactions = 
        srv.shared.database.get_transactions_on_blocks(block_height..block_height + 1)?
        .unwrap();
    let transactions = transactions.first().unwrap();

    let stringified_transactions = txs_stringify(transactions.clone())?; // To be used at check_transition(_, _, transitions)
    
    let block_id = check_transition(
        stringified_initial_state.as_str(), 
        stringified_block.as_str(), 
        stringified_transactions.as_str(),
    );
    
    srv.stop_and_await().await.expect("Could not shutdown node");

    assert_eq!(block.id(), block_id);

    Ok(())
}