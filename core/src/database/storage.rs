use std::{
    borrow::Cow,
    ops::Deref,
};

use fuel_storage::{Mappable, MerkleRoot, StorageInspect, StorageMutate};
use fuel_tx::TxId;
use fuel_types::{BlockHeight, ContractId, Nonce};
use fuel_vm::fuel_merkle::{binary, sparse};
use fuel_vm::storage::ContractsInfo;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::fuel_core_storage::{Error as StorageError, Result as StorageResult};
use crate::database::{Database, Column};
use crate::primitives::BlockId;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct DenseMerkleMetadata {
    /// The root hash of the dense Merkle tree structure
    pub root: MerkleRoot,
    /// The version of the dense Merkle tree structure is equal to the number of
    /// leaves. Every time we append a new leaf to the Merkle tree data set, we
    /// increment the version number.
    pub version: u64,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SparseMerkleMetadata {
    /// The root hash of the sparse Merkle tree structure
    pub root: MerkleRoot,
}

pub struct FuelBlockSecondaryKeyBlockHeights;

impl Mappable for FuelBlockSecondaryKeyBlockHeights {
    /// Secondary key - `BlockHeight`.
    type Key = BlockHeight;
    type OwnedKey = Self::Key;
    /// Primary key - `BlockId`.
    type Value = BlockId;
    type OwnedValue = Self::Value;
}

pub struct FuelBlockMerkleData;

impl Mappable for FuelBlockMerkleData {
    type Key = u64;
    type OwnedKey = Self::Key;
    type Value = binary::Primitive;
    type OwnedValue = Self::Value;
}

pub struct FuelBlockMerkleMetadata;

impl Mappable for FuelBlockMerkleMetadata {
    type Key = BlockHeight;
    type OwnedKey = Self::Key;
    type Value = DenseMerkleMetadata;
    type OwnedValue = Self::Value;
}

pub struct ContractsAssetsMerkleData;

impl Mappable for ContractsAssetsMerkleData {
    type Key = [u8; 32];
    type OwnedKey = Self::Key;
    type Value = sparse::Primitive;
    type OwnedValue = Self::Value;
}

pub struct ContractsAssetsMerkleMetadata;

impl Mappable for ContractsAssetsMerkleMetadata {
    type Key = ContractId;
    type OwnedKey = Self::Key;
    type Value = SparseMerkleMetadata;
    type OwnedValue = Self::Value;
}

pub struct ContractsStateMerkleData;

impl Mappable for ContractsStateMerkleData {
    type Key = [u8; 32];
    type OwnedKey = Self::Key;
    type Value = sparse::Primitive;
    type OwnedValue = Self::Value;
}

pub struct ContractsStateMerkleMetadata;

impl Mappable for ContractsStateMerkleMetadata {
    type Key = ContractId;
    type OwnedKey = Self::Key;
    type Value = SparseMerkleMetadata;
    type OwnedValue = Self::Value;
}

pub trait DatabaseColumn {
    /// The column of the table.
    fn column() -> Column;
}

impl DatabaseColumn for FuelBlockSecondaryKeyBlockHeights {
    fn column() -> Column {
        Column::FuelBlockSecondaryKeyBlockHeights
    }
}

impl DatabaseColumn for FuelBlockMerkleData {
    fn column() -> Column {
        Column::FuelBlockMerkleData
    }
}

impl DatabaseColumn for FuelBlockMerkleMetadata {
    fn column() -> Column {
        Column::FuelBlockMerkleMetadata
    }
}

impl DatabaseColumn for ContractsAssetsMerkleData {
    fn column() -> Column {
        Column::ContractsAssetsMerkleData
    }
}

impl DatabaseColumn for ContractsAssetsMerkleMetadata {
    fn column() -> Column {
        Column::ContractsAssetsMerkleMetadata
    }
}

impl DatabaseColumn for ContractsStateMerkleData {
    fn column() -> Column {
        Column::ContractsStateMerkleData
    }
}

impl DatabaseColumn for ContractsStateMerkleMetadata {
    fn column() -> Column {
        Column::ContractsStateMerkleMetadata
    }
}

impl DatabaseColumn for ContractsInfo {
    fn column() -> Column {
        Column::ContractsInfo
    }
}

impl<T> StorageInspect<T> for Database
where
    T: Mappable + DatabaseColumn,
    T::Key: ToDatabaseKey,
    T::OwnedValue: DeserializeOwned,
{
    type Error = StorageError;

    fn get(&self, key: &T::Key) -> StorageResult<Option<Cow<T::OwnedValue>>> {
        self.get(key.database_key().as_ref(), T::column())
            .map_err(Into::into)
    }

    fn contains_key(&self, key: &T::Key) -> StorageResult<bool> {
        self.contains_key(key.database_key().as_ref(), T::column())
            .map_err(Into::into)
    }
}

impl<T> StorageMutate<T> for Database
where
    T: Mappable + DatabaseColumn,
    T::Key: ToDatabaseKey,
    T::Value: Serialize,
    T::OwnedValue: DeserializeOwned,
{
    fn insert(
        &mut self,
        key: &T::Key,
        value: &T::Value,
    ) -> StorageResult<Option<T::OwnedValue>> {
        Database::insert(self, key.database_key().as_ref(), T::column(), &value)
            .map_err(Into::into)
    }

    fn remove(&mut self, key: &T::Key) -> StorageResult<Option<T::OwnedValue>> {
        Database::remove(self, key.database_key().as_ref(), T::column())
            .map_err(Into::into)
    }
}

pub trait ToDatabaseKey {
    /// A new type of prepared database key that can be converted into bytes.
    type Type<'a>: AsRef<[u8]>
    where
        Self: 'a;

    /// Coverts the key into database key that supports byte presentation.
    fn database_key(&self) -> Self::Type<'_>;
}

impl ToDatabaseKey for BlockHeight {
    type Type<'a> = [u8; 4];

    fn database_key(&self) -> Self::Type<'_> {
        self.to_bytes()
    }
}

impl ToDatabaseKey for u64 {
    type Type<'a> = [u8; 8];

    fn database_key(&self) -> Self::Type<'_> {
        self.to_be_bytes()
    }
}

impl ToDatabaseKey for Nonce {
    type Type<'a> = &'a [u8; 32];

    fn database_key(&self) -> Self::Type<'_> {
        self.deref()
    }
}

impl ToDatabaseKey for ContractId {
    type Type<'a> = &'a [u8; 32];

    fn database_key(&self) -> Self::Type<'_> {
        self.deref()
    }
}

impl ToDatabaseKey for BlockId {
    type Type<'a> = &'a [u8];

    fn database_key(&self) -> Self::Type<'_> {
        self.as_slice()
    }
}

impl ToDatabaseKey for TxId {
    type Type<'a> = &'a [u8; 32];

    fn database_key(&self) -> Self::Type<'_> {
        self.deref()
    }
}

impl ToDatabaseKey for () {
    type Type<'a> = &'a [u8];

    fn database_key(&self) -> Self::Type<'_> {
        &[]
    }
}

impl<const N: usize> ToDatabaseKey for [u8; N] {
    type Type<'a> = &'a [u8];

    fn database_key(&self) -> Self::Type<'_> {
        self.as_slice()
    }
}
