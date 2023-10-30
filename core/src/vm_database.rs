use std::borrow::Cow;
use anyhow::anyhow;

use fuel_tx::{Contract, StorageSlot};
use fuel_types::{BlockHeight, Word, ContractId, Bytes32, Salt};
use fuel_vm::storage::{InterpreterStorage, ContractsAssetsStorage};
use fuel_storage::{MerkleRoot, MerkleRootStorage, Mappable, StorageMutate, StorageInspect, StorageSize, StorageRead};
use primitive_types::U256;
use crate::fuel_core_storage::Error as StorageError;
use crate::database::Database;

#[derive(Clone, Debug)]
pub struct VmDatabase {
    pub block_height: BlockHeight,
    pub coinbase: ContractId,
    pub database: Database
}

trait IncreaseStorageKey {
    fn increase(&mut self) -> anyhow::Result<()>;
}

impl IncreaseStorageKey for U256 {
    fn increase(&mut self) -> anyhow::Result<()> {
        *self = self
            .checked_add(1.into())
            .ok_or_else(|| anyhow!("range op exceeded available keyspace"))?;
        Ok(())
    }
}

impl Default for VmDatabase {
    fn default() -> Self {
        Self {
            block_height: Default::default(),
            coinbase: Default::default(),
            database: Default::default(),
        }
    }
}

impl VmDatabase {
    pub fn new<T>(
        database: Database,
        block_height: BlockHeight,
        coinbase: ContractId,
    ) -> Self {
        Self {
            block_height,
            coinbase,
            database,
        }
    }

    pub fn default_from_database(database: Database) -> Self {
        Self {
            database,
            ..Default::default()
        }
    }

    pub fn database_mut(&mut self) -> &mut Database {
        &mut self.database
    }
}

impl<M: Mappable> StorageInspect<M> for VmDatabase
where
    Database: StorageInspect<M, Error = StorageError>,
{
    type Error = StorageError;

    fn get(&self, key: &M::Key) -> Result<Option<Cow<M::OwnedValue>>, Self::Error> {
        // StorageInspect::<M>::get(&self.database, key)
        todo!()
    }

    fn contains_key(&self, key: &M::Key) -> Result<bool, Self::Error> {
        // StorageInspect::<M>::contains_key(&self.database, key)
        todo!()
    }
}

impl<M: Mappable> StorageMutate<M> for VmDatabase
where
    Database: StorageMutate<M, Error = StorageError>,
{
    fn insert(
        &mut self,
        key: &M::Key,
        value: &M::Value,
    ) -> Result<Option<M::OwnedValue>, Self::Error> {
        StorageMutate::<M>::insert(&mut self.database, key, value)
    }

    fn remove(&mut self, key: &M::Key) -> Result<Option<M::OwnedValue>, Self::Error> {
        StorageMutate::<M>::remove(&mut self.database, key)
    }
}

impl<M: Mappable> StorageSize<M> for VmDatabase
where
    Database: StorageSize<M, Error = StorageError>,
{
    fn size_of_value(&self, key: &M::Key) -> Result<Option<usize>, Self::Error> {
        StorageSize::<M>::size_of_value(&self.database, key)
    }
}

impl<M: Mappable> StorageRead<M> for VmDatabase
where
    Database: StorageRead<M, Error = StorageError>,
{
    fn read(&self, key: &M::Key, buf: &mut [u8]) -> Result<Option<usize>, Self::Error> {
        StorageRead::<M>::read(&self.database, key, buf)
    }

    fn read_alloc(
        &self,
        key: &<M as Mappable>::Key,
    ) -> Result<Option<Vec<u8>>, Self::Error> {
        StorageRead::<M>::read_alloc(&self.database, key)
    }
}

impl<K, M: Mappable> MerkleRootStorage<K, M> for VmDatabase
where
    Database: MerkleRootStorage<K, M, Error = StorageError>,
{
    fn root(&self, key: &K) -> Result<MerkleRoot, Self::Error> {
        MerkleRootStorage::<K, M>::root(&self.database, key)
    }
}

impl ContractsAssetsStorage for VmDatabase {}

impl InterpreterStorage for VmDatabase {
    type DataError = StorageError;

    fn block_height(&self) -> Result<BlockHeight, Self::DataError> {
        todo!()
    }

    fn timestamp(&self, height: BlockHeight) -> Result<Word, Self::DataError> {
        todo!()
    }

    fn block_hash(&self, block_height: BlockHeight) -> Result<Bytes32, Self::DataError> {
        todo!()
    }

    fn coinbase(&self) -> Result<ContractId, Self::DataError> {
        Ok(self.coinbase)
    }

    fn deploy_contract_with_id(
        &mut self,
        salt: &Salt,
        slots: &[StorageSlot],
        contract: &Contract,
        root: &Bytes32,
        id: &ContractId,
    ) -> Result<(), Self::DataError> {
        self.storage_contract_insert(id, contract)?;
        self.storage_contract_root_insert(id, salt, root)?;

        self.database.init_contract_state(
            id,
            slots.iter().map(|slot| (*slot.key(), *slot.value())),
        )
    }

    fn merkle_contract_state_range(
        &self,
        contract_id: &ContractId,
        start_key: &Bytes32,
        range: Word,
    ) -> Result<Vec<Option<Cow<Bytes32>>>, Self::DataError> {
        todo!()
    }

    fn merkle_contract_state_insert_range(
        &mut self,
        contract_id: &ContractId,
        start_key: &Bytes32,
        values: &[Bytes32],
    ) -> Result<Option<()>, Self::DataError> {
        todo!()
    }

    fn merkle_contract_state_remove_range(
        &mut self,
        contract_id: &ContractId,
        start_key: &Bytes32,
        range: Word,
    ) -> Result<Option<()>, Self::DataError> {
        todo!()
    }
}