mod helpers;

pub use std::ops::Deref;

pub use fuel_core::producer::ports::BlockProducerDatabase;
pub use fuel_core::service::ServiceTrait;
pub use fuel_core::types::blockchain::block::Block;
pub use fuels::{tx::Bytes32, accounts::wallet::WalletUnlocked, programs::contract::CallParameters};
pub use prover_core::check_transition;


pub use crate::helpers::{
    bootstrap1, 
    snapshot, 
    SnapshotStringify, 
    block_stringify, 
    txs_stringify, 
    send_funds, 
    get_wallet_by_name, 
    AccountName,
    deploy_smart_wallet,
    WalletContract
};

pub const TEST_SNAPSHOT: &str = include_str!("../res/test_snapshot.json");
pub const TEST_BLOCK: &str = include_str!("../res/test_target_block.json");
pub const TEST_TRANSACTION: &str = include_str!("../res/test_transaction.json");

// /**
//  * This test simulates a simple utxo transfer
//  */
// #[tokio::test]
// async fn test_one_transaction_with_artifacts() -> anyhow::Result<()> {
//     let block: Block<Bytes32> = 
//         serde_json::from_str(TEST_BLOCK)
//         .expect("Could not parse target Block");

//     let result_block = check_transition(
//         TEST_SNAPSHOT,
//         TEST_BLOCK,
//         TEST_TRANSACTION
//     );

//     assert_eq!(block.id(), result_block.id());

//     Ok(())
// }

/**
 * This test simulates two UTXO transfers and a coinbase mint
 */
#[tokio::test]
async fn test_two_transfers() -> anyhow::Result<()> {

    let (srv, provider) = bootstrap1().await.expect("Could not bootstrap node");

    let initial_state = snapshot(&srv)?;
    let stringified_initial_state = initial_state.stringify()?; // To be used at check_transition(state, _, _)
    let initial_block = srv.shared.database.get_current_block()?.unwrap();
    let initial_block_stringified = block_stringify(&initial_block)?;
    
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
    
    let result_block = check_transition(
        stringified_initial_state.as_str(), 
        stringified_block.as_str(), 
        stringified_transactions.as_str(),
        initial_block_stringified.as_str()
    );
    
    srv.stop_and_await().await.expect("Could not shutdown node");

    assert_eq!(block.id(), result_block.id());

    Ok(())
}

#[tokio::test]
async fn test_intermediate_state_transfers() -> anyhow::Result<()> {

    let (srv, provider) = bootstrap1().await.expect("Could not bootstrap node");
    
    
    
    send_funds(&provider, None, None, true).await?;
    
    let initial_state = snapshot(&srv)?;
    let stringified_initial_state = initial_state.stringify()?; // To be used at check_transition(state, _, _)
    let initial_block = srv.shared.database.get_current_block()?.unwrap();
    let initial_block_stringified = block_stringify(&initial_block)?;
    

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
    
    let result_block = check_transition(
        stringified_initial_state.as_str(), 
        stringified_block.as_str(), 
        stringified_transactions.as_str(),
        initial_block_stringified.as_str()
    );
    
    srv.stop_and_await().await.expect("Could not shutdown node");

    assert_eq!(block.id(), result_block.id());

    Ok(())
}

#[tokio::test]
async fn test_deployment_transaction() -> anyhow::Result<()> {
    let (srv, provider) = bootstrap1().await.expect("Could not bootstrap node");

    let initial_state = snapshot(&srv)?;
    let stringified_initial_state = initial_state.stringify()?;
    let initial_block = srv.shared.database.get_current_block()?.unwrap();
    let initial_block_stringified = block_stringify(&initial_block)?;
    

    let mut deployer = get_wallet_by_name(AccountName::Alice);
    deployer.set_provider(provider);
    deploy_smart_wallet(&deployer).await.expect("Could not deploy smart wallet");

    let block = srv.shared.database.get_current_block()?.unwrap();
    let stringified_block = block_stringify(&block)?; // To be used at check_transition(_, block, _)
    
    let block_height = block.header().height().deref().clone();
    let transactions = 
        srv.shared.database.get_transactions_on_blocks(block_height..block_height + 1)?
        .unwrap();
    let transactions = transactions.first().unwrap();

    let stringified_transactions = txs_stringify(transactions.clone())?; // To be used at check_transition(_, _, transitions)
    
    let result_block = check_transition(
        stringified_initial_state.as_str(), 
        stringified_block.as_str(), 
        stringified_transactions.as_str(),
        initial_block_stringified.as_str()
    );

    assert_eq!(block.id(), result_block.id());    

    srv.stop_and_await().await.expect("Could not shutdown node");
    Ok(())
}

// Next: need to import state? This is failing
#[tokio::test]
async fn test_contract_interaction() -> anyhow::Result<()> {
    let (srv, provider) = bootstrap1().await.expect("Could not bootstrap node");

    let mut deployer = get_wallet_by_name(AccountName::Alice);
    deployer.set_provider(provider);
    let contract: WalletContract<WalletUnlocked> = deploy_smart_wallet(&deployer).await.expect("Could not deploy smart wallet");
    
    let initial_state = snapshot(&srv)?;
    let stringified_initial_state = initial_state.clone().stringify()?;
    let initial_block = srv.shared.database.get_current_block()?.unwrap();
    let initial_block_stringified = block_stringify(&initial_block)?;
    

    dbg!(srv.shared.database.get_current_block()?.unwrap());
    contract
        .methods()
        .receive_funds()
        .call_params(
            CallParameters::default()
                .with_asset_id(Default::default())
                .with_amount(100)
        )?
        .call()
        .await?;

    
    let block = srv.shared.database.get_current_block()?.unwrap();
    let stringified_block = block_stringify(&block)?; // To be used at check_transition(_, block, _)
    
    let block_height = block.header().height().deref().clone();
    let transactions = 
        srv.shared.database.get_transactions_on_blocks(block_height..block_height + 1)?
        .unwrap();
    
    let transactions = transactions.first().unwrap();

    let stringified_transactions = txs_stringify(transactions.clone())?; // To be used at check_transition(_, _, transitions)
    
    let result_block = check_transition(
        stringified_initial_state.as_str(), 
        stringified_block.as_str(), 
        stringified_transactions.as_str(),
        initial_block_stringified.as_str()
    );

    dbg!(block.header());
    dbg!(result_block.header());

    assert_eq!(block.id(), result_block.id());    



    srv.stop_and_await().await.expect("Could not shutdown node");
    Ok(())
}