use std::borrow::Cow;
use anyhow::anyhow;

use fuel_tx::{Contract, StorageSlot};
use fuel_types::{Address, BlockHeight, Word, ContractId, Bytes32, Salt};
use fuel_vm::storage::{InterpreterStorage, ContractsAssetsStorage, ContractsAssets};
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

// impl<K, M: Mappable> MerkleRootStorage<K, M> for VmDatabase
// where
//     Database: MerkleRootStorage<K, M, Error = StorageError>,
// {
    // fn root(&self, key: &K) -> Result<MerkleRoot, Self::Error> {
    //     MerkleRootStorage::<K, M>::root(&self.database, key)
    // }
// }

impl MerkleRootStorage<ContractId, ContractsAssets> for VmDatabase
{
    fn root(&self, key: &ContractId) -> Result<MerkleRoot, Self::Error> {
        // MerkleRootStorage::<ContractId, ContractsAssets>::root(&self.database, key)

        todo!()
    }
}

impl ContractsAssetsStorage for VmDatabase {}

// impl InterpreterStorage for VmDatabase {
//     type DataError = StorageError;

//     fn block_height(&self) -> Result<BlockHeight, Self::DataError> {
//         Ok(self.current_block_height)
//     }

//     fn timestamp(&self, height: BlockHeight) -> Result<Word, Self::DataError> {
//         let timestamp = match height {
//             // panic if $rB is greater than the current block height.
//             height if height > self.current_block_height => {
//                 return Err(anyhow!("block height too high for timestamp").into())
//             }
//             height if height == self.current_block_height => self.current_timestamp,
//             height => self.database.block_time(&height)?,
//         };
//         Ok(timestamp.0)
//     }

//     fn block_hash(&self, block_height: BlockHeight) -> Result<Bytes32, Self::DataError> {
//         // Block header hashes for blocks with height greater than or equal to current block height are zero (0x00**32).
//         // https://github.com/FuelLabs/fuel-specs/blob/master/specs/vm/instruction_set.md#bhsh-block-hash
//         if block_height >= self.current_block_height || block_height == Default::default()
//         {
//             Ok(Bytes32::zeroed())
//         } else {
//             // this will return 0x00**32 for block height 0 as well
//             self.database
//                 .get_block_id(&block_height)?
//                 .ok_or(not_found!("BlockId"))
//                 .map(Into::into)
//         }
//     }

//     fn coinbase(&self) -> Result<ContractId, Self::DataError> {
//         Ok(self.coinbase)
//     }

//     fn deploy_contract_with_id(
//         &mut self,
//         salt: &Salt,
//         slots: &[StorageSlot],
//         contract: &Contract,
//         root: &Bytes32,
//         id: &ContractId,
//     ) -> Result<(), Self::DataError> {
//         self.storage_contract_insert(id, contract)?;
//         self.storage_contract_root_insert(id, salt, root)?;

//         self.database.init_contract_state(
//             id,
//             slots.iter().map(|slot| (*slot.key(), *slot.value())),
//         )
//     }

//     fn merkle_contract_state_range(
//         &self,
//         contract_id: &ContractId,
//         start_key: &Bytes32,
//         range: Word,
//     ) -> Result<Vec<Option<Cow<Bytes32>>>, Self::DataError> {
//         // TODO: Optimization: Iterate only over `range` elements.
//         let mut iterator = self.database.iter_all_filtered::<Vec<u8>, Bytes32, _, _>(
//             Column::ContractsState,
//             Some(contract_id),
//             Some(ContractsStateKey::new(contract_id, start_key)),
//             Some(IterDirection::Forward),
//         );
//         let range = range as usize;

//         let mut expected_key = U256::from_big_endian(start_key.as_ref());
//         let mut results = vec![];

//         while results.len() < range {
//             let entry = iterator.next().transpose()?;

//             if entry.is_none() {
//                 // We out of `contract_id` prefix
//                 break
//             }

//             let (multikey, value) =
//                 entry.expect("We did a check before, so the entry should be `Some`");
//             let actual_key = U256::from_big_endian(&multikey[32..]);

//             while (expected_key <= actual_key) && results.len() < range {
//                 if expected_key == actual_key {
//                     // We found expected key, put value into results
//                     results.push(Some(Cow::Owned(value)));
//                 } else {
//                     // Iterator moved beyond next expected key, push none until we find the key
//                     results.push(None);
//                 }
//                 expected_key.increase()?;
//             }
//         }

//         // Fill not initialized slots with `None`.
//         while results.len() < range {
//             results.push(None);
//             expected_key.increase()?;
//         }

//         Ok(results)
//     }

//     fn merkle_contract_state_insert_range(
//         &mut self,
//         contract_id: &ContractId,
//         start_key: &Bytes32,
//         values: &[Bytes32],
//     ) -> Result<Option<()>, Self::DataError> {
//         let mut current_key = U256::from_big_endian(start_key.as_ref());
//         // verify key is in range
//         current_key
//             .checked_add(U256::from(values.len()))
//             .ok_or_else(|| {
//                 DatabaseError::Other(anyhow!("range op exceeded available keyspace"))
//             })?;

//         let mut key_bytes = Bytes32::zeroed();
//         let mut found_unset = false;
//         for value in values {
//             current_key.to_big_endian(key_bytes.as_mut());

//             let option = self
//                 .database
//                 .storage::<ContractsState>()
//                 .insert(&(contract_id, &key_bytes).into(), value)?;

//             found_unset |= option.is_none();

//             current_key.increase()?;
//         }

//         if found_unset {
//             Ok(None)
//         } else {
//             Ok(Some(()))
//         }
//     }

//     fn merkle_contract_state_remove_range(
//         &mut self,
//         contract_id: &ContractId,
//         start_key: &Bytes32,
//         range: Word,
//     ) -> Result<Option<()>, Self::DataError> {
//         let mut found_unset = false;

//         let mut current_key = U256::from_big_endian(start_key.as_ref());

//         let mut key_bytes = Bytes32::zeroed();
//         for _ in 0..range {
//             current_key.to_big_endian(key_bytes.as_mut());

//             let option = self
//                 .database
//                 .storage::<ContractsState>()
//                 .remove(&(contract_id, &key_bytes).into())?;

//             found_unset |= option.is_none();

//             current_key.increase()?;
//         }

//         if found_unset {
//             Ok(None)
//         } else {
//             Ok(Some(()))
//         }
//     }
// }