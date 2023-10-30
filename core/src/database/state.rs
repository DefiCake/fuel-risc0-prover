use std::borrow::Cow;

use fuel_storage::{StorageInspect, Mappable, StorageMutate, MerkleRootStorage, MerkleRoot};
use fuel_types::{ContractId, Bytes32};
use fuel_vm::storage::ContractsState;

use crate::fuel_core_storage::{Error as StorageError, Result as StorageResult};
use crate::database::{Database, Column};

impl StorageInspect<ContractsState> for Database {
    type Error = StorageError;

    fn get(
        &self,
        key: &<ContractsState as Mappable>::Key,
    ) -> Result<Option<Cow<<ContractsState as Mappable>::OwnedValue>>, Self::Error> {
        self.get(key.as_ref(), Column::ContractsState)
            .map_err(Into::into)
    }

    fn contains_key(
        &self,
        key: &<ContractsState as Mappable>::Key,
    ) -> Result<bool, Self::Error> {
        self.contains_key(key.as_ref(), Column::ContractsState)
            .map_err(Into::into)
    }
}

impl StorageMutate<ContractsState> for Database {
    fn insert(
        &mut self,
        key: &<ContractsState as Mappable>::Key,
        value: &<ContractsState as Mappable>::Value,
    ) -> Result<Option<<ContractsState as Mappable>::OwnedValue>, Self::Error> {
        todo!()
    }

    fn remove(
        &mut self,
        key: &<ContractsState as Mappable>::Key,
    ) -> Result<Option<<ContractsState as Mappable>::OwnedValue>, Self::Error> {
        todo!()
    }
}

impl MerkleRootStorage<ContractId, ContractsState> for Database {
    fn root(&self, parent: &ContractId) -> Result<MerkleRoot, Self::Error> {
        todo!()
    }
}

impl Database {
    /// Initialize the state of the contract from all leaves.
    /// This method is more performant than inserting state one by one.
    pub fn init_contract_state<S>(
        &mut self,
        contract_id: &ContractId,
        slots: S,
    ) -> Result<(), StorageError>
    where
        S: Iterator<Item = (Bytes32, Bytes32)>,
    {
        todo!()
    }
}
