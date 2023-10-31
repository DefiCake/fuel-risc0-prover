use crate::{
    serialization::{
        HexNumber,
        HexType,
    }, genesis::GenesisCommitment,
    // GenesisCommitment,
};
use fuel_crypto::Hasher;
use fuel_storage::MerkleRoot;
use fuel_tx::{TxPointer, UtxoId};
use fuel_types::{BlockHeight, Bytes32, Address, AssetId, Word};
use serde::{
    Deserialize,
    Serialize,
};
use serde_with::{
    serde_as,
    skip_serializing_none,
};

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct CoinConfig {
    /// auto-generated if None
    #[serde_as(as = "Option<HexType>")]
    #[serde(default)]
    pub tx_id: Option<Bytes32>,
    #[serde_as(as = "Option<HexNumber>")]
    #[serde(default)]
    pub output_index: Option<u8>,
    /// used if coin is forked from another chain to preserve id & tx_pointer
    #[serde_as(as = "Option<HexNumber>")]
    #[serde(default)]
    pub tx_pointer_block_height: Option<BlockHeight>,
    /// used if coin is forked from another chain to preserve id & tx_pointer
    /// The index of the originating tx within `tx_pointer_block_height`
    #[serde_as(as = "Option<HexNumber>")]
    #[serde(default)]
    pub tx_pointer_tx_idx: Option<u16>,
    #[serde_as(as = "Option<HexNumber>")]
    #[serde(default)]
    pub maturity: Option<BlockHeight>,
    #[serde_as(as = "HexType")]
    pub owner: Address,
    #[serde_as(as = "HexNumber")]
    pub amount: u64,
    #[serde_as(as = "HexType")]
    pub asset_id: AssetId,
}

/// Represents the user's coin for some asset with `asset_id`.
#[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Coin {
    /// The coin utxo id.
    pub utxo_id: UtxoId,
    /// The address with permission to spend this coin
    pub owner: Address,
    /// Amount of coins
    pub amount: Word,
    /// Different incompatible coins can coexist with different asset ids.
    /// This is the "color" of the coin.
    pub asset_id: AssetId,
    /// This coin cannot be spent until the given height
    pub maturity: BlockHeight,
    /// Indexes the block and transaction this coin originated from
    pub tx_pointer: TxPointer,
}

impl Coin {
    /// Compress the coin to minimize the serialized size.
    pub fn compress(self) -> CompressedCoin {
        CompressedCoin {
            owner: self.owner,
            amount: self.amount,
            asset_id: self.asset_id,
            maturity: self.maturity,
            tx_pointer: self.tx_pointer,
        }
    }
}

/// The compressed version of the `Coin` with minimum fields required for
/// the proper work of the blockchain.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompressedCoin {
    /// The address with permission to spend this coin
    pub owner: Address,
    /// Amount of coins
    pub amount: Word,
    /// Different incompatible coins can coexist with different asset ids.
    /// This is the "color" of the coin.
    pub asset_id: AssetId,
    /// This coin cannot be spent until the given height
    pub maturity: BlockHeight,
    /// Indexes the block and transaction this coin originated from
    pub tx_pointer: TxPointer,
}

impl CompressedCoin {
    /// Uncompress the coin.
    pub fn uncompress(self, utxo_id: UtxoId) -> Coin {
        Coin {
            utxo_id,
            owner: self.owner,
            amount: self.amount,
            asset_id: self.asset_id,
            maturity: self.maturity,
            tx_pointer: self.tx_pointer,
        }
    }
}


impl GenesisCommitment for CompressedCoin {
    fn root(&self) -> anyhow::Result<MerkleRoot> {
        let Self {
            owner,
            amount,
            asset_id,
            maturity,
            tx_pointer,
        } = self;

        let coin_hash = *Hasher::default()
            .chain(owner)
            .chain(amount.to_be_bytes())
            .chain(asset_id)
            .chain((*maturity).to_be_bytes())
            .chain(tx_pointer.block_height().to_be_bytes())
            .chain(tx_pointer.tx_index().to_be_bytes())
            .finalize();

        Ok(coin_hash)
    }
}
