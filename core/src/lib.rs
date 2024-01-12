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
    blockchain::{block::Block, primitives::BlockId}, services::{executor::ExecutionTypes, block_producer::Components, p2p::Transactions}
};
use fuel_tx::{Script, Transaction};
use fuel_types::{Nonce, Bytes32};
use genesis::maybe_initialize_state;
use serde::{Deserialize, Serialize};

use fuel_core_executor::{executor::{Executor, OnceTransactionsSource}, ports::RelayerPort};

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
}

pub fn check_transition(
    initial_chain_config_json: &str, 
    target_block_json: &str, 
    transactions_json: &str
) -> BlockId {   
    let config: ChainConfig = 
        serde_json::from_str(initial_chain_config_json)
        .expect("Could not parse ChainConfig JSON");

    let initial_state = config.clone().initial_state.expect("Could not load initial state");
    let initial_height = initial_state.height.expect("Could not load initial height");
    let database = Database::in_memory();
    database.init(&config).expect("database.init() failed");
    maybe_initialize_state(&config, &database).expect("Failed to initialize state");

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

    let component: ExecutionTypes<Components<OnceTransactionsSource>, Block> = ExecutionTypes::Production(Components {
        header_to_produce: reproduced_block_header,
        transactions_source: OnceTransactionsSource::new(transactions.0),
        gas_limit: u64::MAX
    });

    let execution_result = executor.execute_without_commit(
    component,
    Default::default()
    ).expect("Could not get execution result");

    execution_result.result().block.header().hash()
}