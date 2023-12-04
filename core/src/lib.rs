pub mod database;
pub mod fuel_core_storage_custom;
pub mod fuel_core_database;
pub mod primitives;
pub mod config;
pub mod serialization;
pub mod genesis;
pub mod state;
pub mod executor;

use fuel_vm::interpreter::{Interpreter, InterpreterParams};
use fuel_tx::Script;
use fuel_core_chain_config::ChainConfig;

use database::{Database, vm_database::VmDatabase};
// use vm_database::VmDatabase;
// use database::Database;

pub fn initialize_interpreter(json: &str) -> Interpreter<VmDatabase, Script> {   
    let config: ChainConfig = serde_json::from_str(json).expect("Could not parse ChainConfig JSON");
    let db = Database::in_memory();

    todo!()
    // let vm_db: VmDatabase = VmDatabase { block_height: Default::default(), coinbase: Default::default(), database: db };

    // let interpreter: Interpreter<VmDatabase, Script> = Interpreter::with_storage(vm_db, InterpreterParams::default());

    // interpreter   
}

#[cfg(test)]
mod tests {
    use crate::initialize_interpreter;

    const TEST_SNAPSHOT: &str = include_str!("../res/test_snapshot.json");

    #[test]
    fn test_initialize_interpreter() {
        todo!();

        let _ = initialize_interpreter(TEST_SNAPSHOT);        
        // let interpreter = initialize_interpreter();

        // let storage = interpreter.as_ref();

        // let block_height = storage.block_height;

        // assert_eq!(block_height, 0.into(), "Interpreter initialization failed");
    }

    
}