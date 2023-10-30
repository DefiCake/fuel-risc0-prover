use crate::fuel_core_storage::{Error as StorageError, Result as StorageResult};
use crate::database::{Database, Column};

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