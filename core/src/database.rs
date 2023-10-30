#![allow(unused, dead_code)]

pub mod contracts;
pub mod balances;

use serde::{
    de::DeserializeOwned,
    Serialize,
};

use strum::EnumCount;

pub use crate::fuel_core_database::Error;
pub type Result<T> = core::result::Result<T, Error>;

type DatabaseError = Error;
type DatabaseResult<T> = Result<T>;



/// Database tables column ids to the corresponding [`fuel_core_storage::Mappable`] table.
#[repr(u32)]
#[derive(
    Copy, Clone, Debug, strum_macros::EnumCount, PartialEq, Eq, enum_iterator::Sequence,
)]
pub enum Column {
    /// The column id of metadata about the blockchain
    Metadata = 0,
    /// See [`ContractsRawCode`](fuel_core_storage::tables::ContractsRawCode)
    ContractsRawCode = 1,
    /// See [`ContractsInfo`](fuel_core_storage::tables::ContractsInfo)
    ContractsInfo = 2,
    /// See [`ContractsState`](fuel_core_storage::tables::ContractsState)
    ContractsState = 3,
    /// See [`ContractsLatestUtxo`](fuel_core_storage::tables::ContractsLatestUtxo)
    ContractsLatestUtxo = 4,
    /// See [`ContractsAssets`](fuel_core_storage::tables::ContractsAssets)
    ContractsAssets = 5,
    /// See [`Coins`](fuel_core_storage::tables::Coins)
    Coins = 6,
    /// The column of the table that stores `true` if `owner` owns `Coin` with `coin_id`
    OwnedCoins = 7,
    /// See [`Transactions`](fuel_core_storage::tables::Transactions)
    Transactions = 8,
    /// Transaction id to current status
    TransactionStatus = 9,
    /// The column of the table of all `owner`'s transactions
    TransactionsByOwnerBlockIdx = 10,
    /// See [`Receipts`](fuel_core_storage::tables::Receipts)
    Receipts = 11,
    /// See [`FuelBlocks`](fuel_core_storage::tables::FuelBlocks)
    FuelBlocks = 12,
    /// See [`FuelBlockSecondaryKeyBlockHeights`](storage::FuelBlockSecondaryKeyBlockHeights)
    FuelBlockSecondaryKeyBlockHeights = 13,
    /// See [`Messages`](fuel_core_storage::tables::Messages)
    Messages = 14,
    /// The column of the table that stores `true` if `owner` owns `Message` with `message_id`
    OwnedMessageIds = 15,
    /// See [`SealedBlockConsensus`](fuel_core_storage::tables::SealedBlockConsensus)
    FuelBlockConsensus = 16,
    /// See [`FuelBlockMerkleData`](storage::FuelBlockMerkleData)
    FuelBlockMerkleData = 17,
    /// See [`FuelBlockMerkleMetadata`](storage::FuelBlockMerkleMetadata)
    FuelBlockMerkleMetadata = 18,
    /// Messages that have been spent.
    /// Existence of a key in this column means that the message has been spent.
    /// See [`SpentMessages`](fuel_core_storage::tables::SpentMessages)
    SpentMessages = 19,
    /// Metadata for the relayer
    /// See [`RelayerMetadata`](fuel_core_relayer::ports::RelayerMetadata)
    RelayerMetadata = 20,
    /// See [`ContractsAssetsMerkleData`](storage::ContractsAssetsMerkleData)
    ContractsAssetsMerkleData = 21,
    /// See [`ContractsAssetsMerkleMetadata`](storage::ContractsAssetsMerkleMetadata)
    ContractsAssetsMerkleMetadata = 22,
    /// See [`ContractsStateMerkleData`](storage::ContractsStateMerkleData)
    ContractsStateMerkleData = 23,
    /// See [`ContractsStateMerkleMetadata`](storage::ContractsStateMerkleMetadata)
    ContractsStateMerkleMetadata = 24,
}

impl Column {
    /// The total count of variants in the enum.
    pub const COUNT: usize = <Self as EnumCount>::COUNT;

    /// Returns the `usize` representation of the `Column`.
    pub fn as_usize(&self) -> usize {
        *self as usize
    }
}

#[derive(Clone, Debug)]
pub struct Database {}

impl Database {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

/// Mutable methods.
impl Database {
    fn insert<K: AsRef<[u8]>, V: Serialize, R: DeserializeOwned>(
        &self,
        key: K,
        column: Column,
        value: &V,
    ) -> DatabaseResult<Option<R>> {
        unimplemented!()
    }

    fn batch_insert<K: AsRef<[u8]>, V: Serialize, S>(
        &self,
        column: Column,
        set: S,
    ) -> DatabaseResult<()>
    where
        S: Iterator<Item = (K, V)>,
    {
        unimplemented!()
    }

    fn remove<V: DeserializeOwned>(
        &self,
        key: &[u8],
        column: Column,
    ) -> DatabaseResult<Option<V>> {
        unimplemented!()
    }

    fn write(&self, key: &[u8], column: Column, buf: &[u8]) -> DatabaseResult<usize> {
        unimplemented!()
    }

    fn replace(
        &self,
        key: &[u8],
        column: Column,
        buf: &[u8],
    ) -> DatabaseResult<(usize, Option<Vec<u8>>)> {
        unimplemented!()
    }

    fn take(&self, key: &[u8], column: Column) -> DatabaseResult<Option<Vec<u8>>> {
        unimplemented!()
    }
}

/// Read-only methods.
impl Database {
    fn contains_key(&self, key: &[u8], column: Column) -> DatabaseResult<bool> {
        unimplemented!()
    }

    fn size_of_value(&self, key: &[u8], column: Column) -> DatabaseResult<Option<usize>> {
        unimplemented!()
    }

    fn read(
        &self,
        key: &[u8],
        column: Column,
        buf: &mut [u8],
    ) -> DatabaseResult<Option<usize>> {
        unimplemented!()
    }

    fn read_alloc(&self, key: &[u8], column: Column) -> DatabaseResult<Option<Vec<u8>>> {
        unimplemented!()
    }

    fn get<V: DeserializeOwned>(
        &self,
        key: &[u8],
        column: Column,
    ) -> DatabaseResult<Option<V>> {
        unimplemented!()
    }
}
