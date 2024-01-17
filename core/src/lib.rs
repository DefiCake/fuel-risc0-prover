pub mod database;
pub mod state;
pub mod genesis;
pub mod executor;
// pub mod config;

use std::sync::Arc;

use fuel_core_chain_config::ChainConfig;
use database::Database;

use fuel_core_types::{
    blockchain::{primitives::DaBlockHeight, header::PartialBlockHeader}, 
    entities::message::Message,
    blockchain::block::{Block, PartialFuelBlock}, services::{executor::ExecutionTypes, block_producer::Components, p2p::Transactions}, fuel_merkle
};
use fuel_tx::Transaction;
use fuel_types::{Nonce, Bytes32, canonical::Serialize as FuelTypesSerialize};
use genesis::initialize_state;
use serde::{Deserialize, Serialize};

use fuel_core_executor::{executor::{Executor, OnceTransactionsSource, ExecutionOptions}, ports::RelayerPort};

#[derive(Clone)]
pub struct MockRelayer {
  database: Database,
}

impl RelayerPort for MockRelayer {
    fn get_message(&self, id: &Nonce, _da_height: &DaBlockHeight) -> anyhow::Result<Option<Message>> {
        use fuel_core_storage::{ tables::Messages, StorageAsRef };
        use std::borrow::Cow;
        Ok(self.database.storage::<Messages>().get(id)?.map(Cow::into_owned))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Inputs {
    pub chain_config: String,
    pub target_block: String,
    pub transactions_json: String,
    pub initial_block_json: String,
}

pub fn check_transition(
    initial_chain_config_json: &str, 
    target_block_json: &str, 
    transactions_json: &str,
    initial_block_json: &str
) -> Block {   
    let config: ChainConfig = 
        serde_json::from_str(initial_chain_config_json)
        .expect("Could not parse ChainConfig JSON");

    let initial_state = config.clone().initial_state.expect("Could not load initial state");
    let initial_height = initial_state.height.expect("Could not load initial height");
    let initial_block: Block = serde_json::from_str(initial_block_json).expect("Could not load initial block");

    let database = Database::in_memory();
    database.init(&config).expect("database.init() failed");
    initialize_state(&config, &database, &initial_block).expect("Failed to initialize state");

    let core_initial_block = database.get_current_block().unwrap().unwrap();
    // dbg!(core_initial_block);

    let relayer: MockRelayer = MockRelayer { database: database.clone() };

    let executor: Executor<MockRelayer, Database> = Executor {
        relayer,
        database: database.clone(),
        config: Arc::new(Default::default()),
    };

    let block: Block<Bytes32> = 
        serde_json::from_str(target_block_json)
        .expect("Could not parse target Block");

    let time = block.header().time();

    let height: fuel_crypto::fuel_types::BlockHeight = (u32::from(initial_height) + 1u32).into();
    let prev_root = block.header().prev_root().clone();

    let transactions: Transactions = 
        serde_json::from_str(transactions_json)
        .expect("Could not deserialize transactions");

    let mut def = PartialBlockHeader::default();
    def.consensus.prev_root = prev_root;
    def.consensus.time = time;
    def.consensus.height = height;

    let reproduced_block_header: PartialBlockHeader = PartialBlockHeader { ..def };

    let component: ExecutionTypes<Components<OnceTransactionsSource>, Block> = ExecutionTypes::DryRun(Components {
        header_to_produce: reproduced_block_header.clone(),
        transactions_source: OnceTransactionsSource::new(transactions.clone().0),
        gas_limit: u64::MAX
    });

    let test_block: PartialFuelBlock = PartialFuelBlock {
        header: reproduced_block_header.clone(),
        transactions: transactions.clone().0
    };

    let test: ExecutionTypes<PartialFuelBlock, Block> = ExecutionTypes::Validation(
        Block::try_from_executed(block.header().clone(), transactions.clone().0).unwrap()
    );

    let execution_result = executor.execute_and_commit(
        test, 
        ExecutionOptions{ utxo_validation: true}
    ).expect("Could not get execution result");

    // let execution_result = executor.execute_without_commit(
    // component,
    // ExecutionOptions { utxo_validation: true }
    // ).expect("Could not get execution result").into_result();

    // dbg!(&execution_result.tx_status);
    let result_block: Block = execution_result.block.clone();
    let mut whatever = result_block.clone();


    let does_not_validate = Block::try_from_executed(execution_result.block.header().clone(), transactions.0);
    // dbg!(does_not_validate);

    result_block
}

fn generate_txns_root(transactions: &[Transaction]) -> Bytes32 {
    let transaction_ids = transactions.iter().map(|tx| tx.to_bytes());
    // Generate the transaction merkle root.
    let mut transaction_tree =
        fuel_merkle::binary::root_calculator::MerkleRootCalculator::new();
    for id in transaction_ids {
        transaction_tree.push(id.as_ref());
    }
    transaction_tree.root().into()
}