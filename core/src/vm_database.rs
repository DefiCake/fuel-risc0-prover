use std::borrow::Cow;

use fuel_tx::{Contract, StorageSlot};
use fuel_types::{Address, BlockHeight, Word, ContractId, Bytes32, Salt};
use fuel_vm::storage::{InterpreterStorage, ContractsAssetsStorage};
use fuel_storage::{MerkleRoot, MerkleRootStorage, StorageError, Mappable, StorageMutate, StorageInspect, StorageSize, StorageRead};

use crate::database::Database;

#[derive(Clone, Debug)]
pub struct VmDatabase {
    pub block_height: BlockHeight,
    pub coinbase: Address,
    pub database: Database
}

trait IncreaseStorageKey {
    fn increase(&mut self) -> anyhow::Result<()>;
}

impl<M: Mappable> StorageInspect<M> for VmDatabase
{
    fn get(&self, _key: &M::Key) -> Result<Option<Cow<M::OwnedValue>>, StorageError> {
        unimplemented!()
    }

    fn contains_key(&self, _key: &M::Key) -> Result<bool, StorageError> {
        unimplemented!()
    }
}

impl<M: Mappable> StorageMutate<M> for VmDatabase
{
    fn insert(
        &mut self,
        _key: &M::Key,
        _value: &M::Value,
    ) -> Result<Option<M::OwnedValue>, StorageError> {
        unimplemented!()
    }

    fn remove(&mut self, _key: &M::Key) -> Result<Option<M::OwnedValue>, StorageError> {
        unimplemented!()
    }
}

impl<M: Mappable> StorageSize<M> for VmDatabase
{
    fn size_of_value(&self, _key: &M::Key) -> Result<Option<usize>, StorageError> {
        unimplemented!()
    }
}

impl<M: Mappable> StorageRead<M> for VmDatabase
{
    fn read(&self, _key: &M::Key, _buf: &mut [u8]) -> Result<Option<usize>, StorageError> {
        unimplemented!()
    }

    fn read_alloc(
        &self,
        _key: &<M as Mappable>::Key,
    ) -> Result<Option<Vec<u8>>, StorageError> {
        unimplemented!()
    }
}

impl<K, M: Mappable> MerkleRootStorage<K, M> for VmDatabase
{
    fn root(&self, _key: &K) -> Result<MerkleRoot, StorageError> {
        unimplemented!()
    }
}

impl ContractsAssetsStorage for VmDatabase {}

impl InterpreterStorage for VmDatabase {

    fn block_height(&self) -> Result<BlockHeight, StorageError> {
        Ok(self.block_height)
    }

    fn timestamp(&self, _height: BlockHeight) -> Result<Word, StorageError> {
        unimplemented!()
    }

    fn block_hash(&self, _block_height: BlockHeight) -> Result<Bytes32, StorageError> {
        unimplemented!()
    }

    fn coinbase(&self) -> Result<Address, StorageError> {
        Ok(self.coinbase)
    }

    fn deploy_contract_with_id(
        &mut self,
        _salt: &Salt,
        _slots: &[StorageSlot],
        _contract: &Contract,
        _root: &Bytes32,
        _id: &ContractId,
    ) -> Result<(), StorageError> {
        unimplemented!()
    }

    fn merkle_contract_state_range(
        &self,
        _contract_id: &ContractId,
        _start_key: &Bytes32,
        _range: Word,
    ) -> Result<Vec<Option<Cow<Bytes32>>>, StorageError> {
        unimplemented!()
    }

    fn merkle_contract_state_insert_range(
        &mut self,
        _contract_id: &ContractId,
        _start_key: &Bytes32,
        _values: &[Bytes32],
    ) -> Result<Option<()>, StorageError> {
        unimplemented!()
    }

    fn merkle_contract_state_remove_range(
        &mut self,
        _contract_id: &ContractId,
        _start_key: &Bytes32,
        _range: Word,
    ) -> Result<Option<()>, StorageError> {
        unimplemented!()
    }
}

// impl Default for VmDatabase {
//     fn default() -> Self {
//         Self {
//             current_block_height: Default::default(),
//             current_timestamp: Tai64::now(),
//             coinbase: Default::default(),
//             database: Default::default(),
//         }
//     }
// }

// impl VmDatabase {
//     pub fn new<T>(
//         database: Database,
//         header: &ConsensusHeader<T>,
//         coinbase: Address,
//     ) -> Self {
//         Self {
//             current_block_height: header.height,
//             current_timestamp: header.time,
//             coinbase,
//             database,
//         }
//     }

//     pub fn database_mut(&mut self) -> &mut Database {
//         &mut self.database
//     }
// }

// impl<M: Mappable> StorageInspect<M> for VmDatabase
// {
//     type Error = StorageError;

//     fn get(&self, key: &M::Key) -> Result<Option<Cow<M::OwnedValue>>, StorageError> {
//         StorageInspect::<M>::get(&self.database, key)
//     }

//     fn contains_key(&self, key: &M::Key) -> Result<bool, StorageError> {
//         StorageInspect::<M>::contains_key(&self.database, key)
//     }
// }

// impl<M: Mappable> StorageMutate<M> for VmDatabase
// {
//     fn insert(
//         &mut self,
//         key: &M::Key,
//         value: &M::Value,
//     ) -> Result<Option<M::OwnedValue>, StorageError> {
//         StorageMutate::<M>::insert(&mut self.database, key, value)
//     }

//     fn remove(&mut self, key: &M::Key) -> Result<Option<M::OwnedValue>, StorageError> {
//         StorageMutate::<M>::remove(&mut self.database, key)
//     }
// }

// impl<M: Mappable> StorageSize<M> for VmDatabase
// {
//     fn size_of_value(&self, key: &M::Key) -> Result<Option<usize>, StorageError> {
//         StorageSize::<M>::size_of_value(&self.database, key)
//     }
// }

// impl<M: Mappable> StorageRead<M> for VmDatabase
// {
//     fn read(&self, key: &M::Key, buf: &mut [u8]) -> Result<Option<usize>, StorageError> {
//         StorageRead::<M>::read(&self.database, key, buf)
//     }

//     fn read_alloc(
//         &self,
//         key: &<M as Mappable>::Key,
//     ) -> Result<Option<Vec<u8>>, StorageError> {
//         StorageRead::<M>::read_alloc(&self.database, key)
//     }
// }

// impl<K, M: Mappable> MerkleRootStorage<K, M> for VmDatabase
// {
//     fn root(&self, key: &K) -> Result<MerkleRoot, StorageError> {
//         MerkleRootStorage::<K, M>::root(&self.database, key)
//     }
// }

// impl ContractsAssetsStorage for VmDatabase {}

// impl InterpreterStorage for VmDatabase {
//     type StorageError = StorageError;

//     fn block_height(&self) -> Result<BlockHeight, StorageError> {
//         Ok(self.current_block_height)
//     }

//     fn timestamp(&self, height: BlockHeight) -> Result<Word, StorageError> {
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

//     fn block_hash(&self, block_height: BlockHeight) -> Result<Bytes32, StorageError> {
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

//     fn coinbase(&self) -> Result<Address, StorageError> {
//         Ok(self.coinbase)
//     }

//     fn deploy_contract_with_id(
//         &mut self,
//         salt: &Salt,
//         slots: &[StorageSlot],
//         contract: &Contract,
//         root: &Bytes32,
//         id: &ContractId,
//     ) -> Result<(), StorageError> {
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
//     ) -> Result<Vec<Option<Cow<Bytes32>>>, StorageError> {
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
//     ) -> Result<Option<()>, StorageError> {
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
//     ) -> Result<Option<()>, StorageError> {
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