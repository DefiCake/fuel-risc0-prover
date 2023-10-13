
pub struct Database {}

// use serde::{
//     de::DeserializeOwned,
//     Serialize,
// };

// pub type Error;
// pub type Result<T> = core::result::Result<T, Error>;

// type DatabaseError = Error;
// type DatabaseResult<T> = Result<T>;


// /// Database tables column ids to the corresponding [`fuel_core_storage::Mappable`] table.
// #[repr(u32)]
// #[derive(
//     Copy, Clone, Debug, strum_macros::EnumCount, PartialEq, Eq, enum_iterator::Sequence,
// )]
// pub enum Column {
//     /// The column id of metadata about the blockchain
//     Metadata = 0,
//     /// See [`ContractsRawCode`](fuel_core_storage::tables::ContractsRawCode)
//     ContractsRawCode = 1,
//     /// See [`ContractsInfo`](fuel_core_storage::tables::ContractsInfo)
//     ContractsInfo = 2,
//     /// See [`ContractsState`](fuel_core_storage::tables::ContractsState)
//     ContractsState = 3,
//     /// See [`ContractsLatestUtxo`](fuel_core_storage::tables::ContractsLatestUtxo)
//     ContractsLatestUtxo = 4,
//     /// See [`ContractsAssets`](fuel_core_storage::tables::ContractsAssets)
//     ContractsAssets = 5,
//     /// See [`Coins`](fuel_core_storage::tables::Coins)
//     Coins = 6,
//     /// The column of the table that stores `true` if `owner` owns `Coin` with `coin_id`
//     OwnedCoins = 7,
//     /// See [`Transactions`](fuel_core_storage::tables::Transactions)
//     Transactions = 8,
//     /// Transaction id to current status
//     TransactionStatus = 9,
//     /// The column of the table of all `owner`'s transactions
//     TransactionsByOwnerBlockIdx = 10,
//     /// See [`Receipts`](fuel_core_storage::tables::Receipts)
//     Receipts = 11,
//     /// See [`FuelBlocks`](fuel_core_storage::tables::FuelBlocks)
//     FuelBlocks = 12,
//     /// See [`FuelBlockSecondaryKeyBlockHeights`](storage::FuelBlockSecondaryKeyBlockHeights)
//     FuelBlockSecondaryKeyBlockHeights = 13,
//     /// See [`Messages`](fuel_core_storage::tables::Messages)
//     Messages = 14,
//     /// The column of the table that stores `true` if `owner` owns `Message` with `message_id`
//     OwnedMessageIds = 15,
//     /// See [`SealedBlockConsensus`](fuel_core_storage::tables::SealedBlockConsensus)
//     FuelBlockConsensus = 16,
//     /// See [`FuelBlockMerkleData`](storage::FuelBlockMerkleData)
//     FuelBlockMerkleData = 17,
//     /// See [`FuelBlockMerkleMetadata`](storage::FuelBlockMerkleMetadata)
//     FuelBlockMerkleMetadata = 18,
//     /// Messages that have been spent.
//     /// Existence of a key in this column means that the message has been spent.
//     /// See [`SpentMessages`](fuel_core_storage::tables::SpentMessages)
//     SpentMessages = 19,
//     /// Metadata for the relayer
//     /// See [`RelayerMetadata`](fuel_core_relayer::ports::RelayerMetadata)
//     RelayerMetadata = 20,
//     /// See [`ContractsAssetsMerkleData`](storage::ContractsAssetsMerkleData)
//     ContractsAssetsMerkleData = 21,
//     /// See [`ContractsAssetsMerkleMetadata`](storage::ContractsAssetsMerkleMetadata)
//     ContractsAssetsMerkleMetadata = 22,
//     /// See [`ContractsStateMerkleData`](storage::ContractsStateMerkleData)
//     ContractsStateMerkleData = 23,
//     /// See [`ContractsStateMerkleMetadata`](storage::ContractsStateMerkleMetadata)
//     ContractsStateMerkleMetadata = 24,
// }

// impl Column {
//     /// The total count of variants in the enum.
//     pub const COUNT: usize = <Self as EnumCount>::COUNT;

//     /// Returns the `usize` representation of the `Column`.
//     pub fn as_usize(&self) -> usize {
//         *self as usize
//     }
// }

// #[derive(Clone, Debug)]
// pub struct Database {}

// impl Database {
//     pub fn new(data_source: DataSource) -> Self {
//         Self {}
//     }
// }

// /// Mutable methods.
// impl Database {
//     fn insert<K: AsRef<[u8]>, V: Serialize, R: DeserializeOwned>(
//         &self,
//         key: K,
//         column: Column,
//         value: &V,
//     ) -> DatabaseResult<Option<R>> {
//         unimplemented!()
//     }

//     fn batch_insert<K: AsRef<[u8]>, V: Serialize, S>(
//         &self,
//         column: Column,
//         set: S,
//     ) -> DatabaseResult<()>
//     where
//         S: Iterator<Item = (K, V)>,
//     {
//         unimplemented!()
//     }

//     fn remove<V: DeserializeOwned>(
//         &self,
//         key: &[u8],
//         column: Column,
//     ) -> DatabaseResult<Option<V>> {
//         unimplemented!()
//     }

//     fn write(&self, key: &[u8], column: Column, buf: &[u8]) -> DatabaseResult<usize> {
//         unimplemented!()
//     }

//     fn replace(
//         &self,
//         key: &[u8],
//         column: Column,
//         buf: &[u8],
//     ) -> DatabaseResult<(usize, Option<Vec<u8>>)> {
//         unimplemented!()
//     }

//     fn take(&self, key: &[u8], column: Column) -> DatabaseResult<Option<Vec<u8>>> {
//         unimplemented!()
//     }
// }

// /// Read-only methods.
// impl Database {
//     fn contains_key(&self, key: &[u8], column: Column) -> DatabaseResult<bool> {
//         unimplemented!()
//     }

//     fn size_of_value(&self, key: &[u8], column: Column) -> DatabaseResult<Option<usize>> {
//         unimplemented!()
//     }

//     fn read(
//         &self,
//         key: &[u8],
//         column: Column,
//         buf: &mut [u8],
//     ) -> DatabaseResult<Option<usize>> {
//         unimplemented!()
//     }

//     fn read_alloc(&self, key: &[u8], column: Column) -> DatabaseResult<Option<Vec<u8>>> {
//         unimplemented!()
//     }

//     fn get<V: DeserializeOwned>(
//         &self,
//         key: &[u8],
//         column: Column,
//     ) -> DatabaseResult<Option<V>> {
//         unimplemented!()
//     }

//     // fn iter_all<K, V>(
//     //     &self,
//     //     column: Column,
//     //     direction: Option<IterDirection>,
//     // ) -> impl Iterator<Item = DatabaseResult<(K, V)>> + '_
//     // where
//     //     K: From<Vec<u8>>,
//     //     V: DeserializeOwned,
//     // {
//     //     unimplemented!()
//     // }

//     // fn iter_all_by_prefix<K, V, P>(
//     //     &self,
//     //     column: Column,
//     //     prefix: Option<P>,
//     // ) -> impl Iterator<Item = DatabaseResult<(K, V)>> + '_
//     // where
//     //     K: From<Vec<u8>>,
//     //     V: DeserializeOwned,
//     //     P: AsRef<[u8]>,
//     // {
//     //     unimplemented!()
//     // }

//     // fn iter_all_by_start<K, V, S>(
//     //     &self,
//     //     column: Column,
//     //     start: Option<S>,
//     //     direction: Option<IterDirection>,
//     // ) -> impl Iterator<Item = DatabaseResult<(K, V)>> + '_
//     // where
//     //     K: From<Vec<u8>>,
//     //     V: DeserializeOwned,
//     //     S: AsRef<[u8]>,
//     // {
//     //     unimplemented!()
//     // }

//     // fn iter_all_filtered<K, V, P, S>(
//     //     &self,
//     //     column: Column,
//     //     prefix: Option<P>,
//     //     start: Option<S>,
//     //     direction: Option<IterDirection>,
//     // ) -> impl Iterator<Item = DatabaseResult<(K, V)>> + '_
//     // where
//     //     K: From<Vec<u8>>,
//     //     V: DeserializeOwned,
//     //     P: AsRef<[u8]>,
//     //     S: AsRef<[u8]>,
//     // {
//     //     unimplemented!()
//     // }
// }

// impl AsRef<Database> for Database {
//     fn as_ref(&self) -> &Database {
//         self
//     }
// }

// /// Construct an ephemeral database
// impl Default for Database {
//     fn default() -> Self {
//         Self {}
//     }
// }

// /// Implement `ChainConfigDb` so that `Database` can be passed to
// /// `StateConfig's` `generate_state_config()` method
// // impl ChainConfigDb for Database {
// //     fn get_coin_config(&self) -> Result<Option<Vec<CoinConfig>>> {
// //         unimplemented!()
// //     }

// //     fn get_contract_config(&self) -> Result<Option<Vec<ContractConfig>>> {
// //         unimplemented!()
// //     }

// //     fn get_message_config(&self) -> Result<Option<Vec<MessageConfig>>> {
// //         unimplemented!()
// //     }

// //     fn get_block_height(&self) -> Result<BlockHeight> {
// //         unimplemented!()
// //     }
// // }


// #[test]
// fn column_keys_not_exceed_count() {
//     use enum_iterator::all;
//     for column in all::<Column>() {
//         assert!(column.as_usize() < Column::COUNT);
//     }
// }
