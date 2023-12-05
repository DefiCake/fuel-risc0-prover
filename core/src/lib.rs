pub mod database;
// pub mod fuel_core_storage_custom;
// pub mod fuel_core_database;
// pub mod primitives;
// pub mod config;
// pub mod serialization;
// pub mod genesis;
pub mod state;
pub mod executor;

use std::sync::Arc;

// use fuel_core_types::{blockchain::primitives::DaBlockHeight, entities::message::Message};
// use fuel_types::Nonce;
// use fuel_vm::interpreter::{Interpreter, InterpreterParams};
// use fuel_tx::Script;
use fuel_core_chain_config::ChainConfig;
use database::Database;
use executor::{Executor, RelayerPort};
use fuel_core_types::{blockchain::primitives::DaBlockHeight, entities::message::Message};
use fuel_types::Nonce;

#[derive(Clone, Debug)]
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

pub fn initialize_interpreter(json: &str) -> Executor<MockRelayer> {   
    let config: ChainConfig = serde_json::from_str(json).expect("Could not parse ChainConfig JSON");

    let initial_state = config.clone().initial_state.expect("Could not load initial state");
    let _initial_height = initial_state.height.expect("Could not load initial height");
    let database = Database::in_memory();
    database.init(&config).expect("database.init() failed");

    let relayer: MockRelayer = MockRelayer { database: database.clone() };

    let executor: Executor<MockRelayer> = Executor {
        relayer,
        database: database.clone(),
        config: Arc::new(Default::default()),
    };

    executor
}

#[cfg(test)]
mod tests {
    use crate::initialize_interpreter;

    const TEST_SNAPSHOT: &str = include_str!("../res/test_snapshot.json");

    #[test]
    fn test_initialize_interpreter() {

        let _executor = initialize_interpreter(TEST_SNAPSHOT);        
        // let interpreter = initialize_interpreter();

        // let storage = interpreter.as_ref();

        // let block_height = storage.block_height;

        // assert_eq!(block_height, 0.into(), "Interpreter initialization failed");
    }

    
}