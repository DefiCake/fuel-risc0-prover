use fuel_core_executor::ports::{MessageIsSpent, TxIdOwnerRecorder};

use crate::database::Database;

use fuel_core_storage::{Error as StorageError, vm_storage::VmStorageRequirements};

impl MessageIsSpent for Database {
    type Error = StorageError;

    fn message_is_spent(&self, nonce: &fuel_types::Nonce) -> Result<bool, fuel_core_storage::Error> {
        todo!()
    }
}

impl VmStorageRequirements for Database {
    type Error = StorageError;

    fn block_time(&self, height: &fuel_types::BlockHeight) -> Result<fuel_core_types::tai64::Tai64, Self::Error> {
        todo!()
    }

    fn get_block_id(&self, height: &fuel_types::BlockHeight) -> Result<Option<fuel_core_types::blockchain::primitives::BlockId>, Self::Error> {
        todo!()
    }

    fn init_contract_state<S: Iterator<Item = (fuel_types::Bytes32, fuel_types::Bytes32)>>(
        &mut self,
        contract_id: &fuel_types::ContractId,
        slots: S,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}

impl TxIdOwnerRecorder for Database {
    type Error = StorageError;

    fn record_tx_id_owner(
        &self,
        owner: &fuel_types::Address,
        block_height: fuel_types::BlockHeight,
        tx_idx: u16,
        tx_id: &fuel_types::Bytes32,
    ) -> Result<Option<fuel_types::Bytes32>, Self::Error> {
        todo!()
    }

    fn update_tx_status(
        &self,
        id: &fuel_types::Bytes32,
        status: fuel_core_types::services::txpool::TransactionStatus,
    ) -> Result<Option<fuel_core_types::services::txpool::TransactionStatus>, Self::Error> {
        todo!()
    }
}