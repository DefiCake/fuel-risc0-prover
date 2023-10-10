use fuel_vm::{storage::MemoryStorage, interpreter::Interpreter};
use fuel_tx::Script;


pub fn initialize_interpreter() -> Interpreter<MemoryStorage, Script>  {
    let interpreter: Interpreter<MemoryStorage, Script> = Interpreter::with_memory_storage();

    interpreter   
}

#[cfg(test)]
mod tests {
    use crate::initialize_interpreter;

    #[test]
    fn test_initialize_interpreter() {
        let interpreter = initialize_interpreter();

        let storage = interpreter.as_ref();

        let contracts_count = storage.all_contract_state().count();

        assert_eq!(contracts_count, 0, "asdadsad");
    }
}