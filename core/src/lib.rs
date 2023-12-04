pub mod vm_database;
pub mod database;
pub mod fuel_core_storage_custom;
pub mod fuel_core_database;
pub mod primitives;
pub mod config;
pub mod serialization;
pub mod genesis;
pub mod state;

use fuel_vm::interpreter::{Interpreter, InterpreterParams};
use fuel_tx::Script;
use fuel_core_chain_config::ChainConfig;

use database::Database;
// use vm_database::VmDatabase;
// use database::Database;

pub fn initialize_interpreter(json: &str)  {   
    let config: ChainConfig = serde_json::from_str(json).expect("Could not parse ChainConfig JSON");
    let database = Database::in_memory();
    // let db: Database = Database::new();

    // let vm_db: VmDatabase = VmDatabase { block_height: Default::default(), coinbase: Default::default(), database: db };

    // let interpreter: Interpreter<VmDatabase, Script> = Interpreter::with_storage(vm_db, InterpreterParams::default());

    // interpreter   
}

#[cfg(test)]
mod tests {
    use crate::initialize_interpreter;

    #[test]
    fn test_initialize_interpreter() {
        // let interpreter = initialize_interpreter();

        // let storage = interpreter.as_ref();

        // let block_height = storage.block_height;

        // assert_eq!(block_height, 0.into(), "Interpreter initialization failed");
    }

    
}