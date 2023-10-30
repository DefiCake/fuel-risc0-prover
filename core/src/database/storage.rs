use fuel_storage::{Mappable, MerkleRoot};
use fuel_types::{BlockHeight, ContractId};
use fuel_vm::fuel_merkle::{binary, sparse};

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