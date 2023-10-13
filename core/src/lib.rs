pub mod vm_database;

use fuel_vm::interpreter::{Interpreter, InterpreterParams};
use fuel_tx::Script;
use vm_database::VmDatabase;



pub fn initialize_interpreter() -> Interpreter<VmDatabase, Script>  {

    let db: VmDatabase = VmDatabase { block_height: Default::default(), coinbase: Default::default() };

    let interpreter: Interpreter<VmDatabase, Script> = Interpreter::with_storage(db, InterpreterParams::default());

    interpreter   
}

#[cfg(test)]
mod tests {
    use crate::initialize_interpreter;

    #[test]
    fn test_initialize_interpreter() {
        let interpreter = initialize_interpreter();

        let storage = interpreter.as_ref();

        let block_height = storage.block_height;

        assert_eq!(block_height, 0.into(), "Interpreter initialization failed");
    }
}