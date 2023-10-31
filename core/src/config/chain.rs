use fuel_crypto::{SecretKey, Hasher};
use fuel_storage::MerkleRoot;
use fuel_tx::{ConsensusParameters, TxParameters, UtxoId, GasCosts};
use fuel_types::{AssetId, Address};
use core::str::FromStr;
use itertools::Itertools;
use serde::{
    Deserialize,
    Serialize,
};
use serde_with::{
    serde_as,
    skip_serializing_none,
};

use crate::{
    config::{
        coin::CoinConfig,
        state::StateConfig,
        consensus::ConsensusConfig
    },
    genesis::GenesisCommitment,
};

// Fuel Network human-readable part for bech32 encoding
pub const FUEL_BECH32_HRP: &str = "fuel";
pub const LOCAL_TESTNET: &str = "local_testnet";
pub const TESTNET_INITIAL_BALANCE: u64 = 10_000_000;

#[serde_as]
// TODO: Remove not consensus/network fields from `ChainConfig` or create a new config only
//  for consensus/network fields.
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct ChainConfig {
    pub chain_name: String,
    pub block_gas_limit: u64,
    #[serde(default)]
    pub initial_state: Option<StateConfig>,
    pub consensus_parameters: ConsensusParameters,
    pub consensus: ConsensusConfig,
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            chain_name: "local".into(),
            block_gas_limit: TxParameters::DEFAULT.max_gas_per_tx * 10, /* TODO: Pick a sensible default */
            consensus_parameters: ConsensusParameters::default(),
            initial_state: None,
            consensus: ConsensusConfig::default_poa(),
        }
    }
}

impl ChainConfig {
    pub const BASE_ASSET: AssetId = AssetId::zeroed();

    pub fn local_testnet() -> Self {
        // endow some preset accounts with an initial balance
        let secrets = [
            "0xde97d8624a438121b86a1956544bd72ed68cd69f2c99555b08b1e8c51ffd511c",
            "0x37fa81c84ccd547c30c176b118d5cb892bdb113e8e80141f266519422ef9eefd",
            "0x862512a2363db2b3a375c0d4bbbd27172180d89f23f2e259bac850ab02619301",
            "0x976e5c3fa620092c718d852ca703b6da9e3075b9f2ecb8ed42d9f746bf26aafb",
            "0x7f8a325504e7315eda997db7861c9447f5c3eff26333b20180475d94443a10c6",
        ];
        let initial_coins = secrets
            .into_iter()
            .map(|secret| {
                let secret = SecretKey::from_str(secret).expect("Expected valid secret");
                Self::initial_coin(secret, TESTNET_INITIAL_BALANCE, None)
            })
            .collect_vec();

        Self {
            chain_name: LOCAL_TESTNET.to_string(),
            initial_state: Some(StateConfig {
                coins: Some(initial_coins),
                ..StateConfig::default()
            }),
            ..Default::default()
        }
    }

    pub fn initial_coin(
        secret: SecretKey,
        amount: u64,
        utxo_id: Option<UtxoId>,
    ) -> CoinConfig {
        let address = Address::from(*secret.public_key().hash());

        CoinConfig {
            tx_id: utxo_id.as_ref().map(|u| *u.tx_id()),
            output_index: utxo_id.as_ref().map(|u| u.output_index()),
            tx_pointer_block_height: None,
            tx_pointer_tx_idx: None,
            maturity: None,
            owner: address,
            amount,
            asset_id: Default::default(),
        }
    }
}


impl GenesisCommitment for ChainConfig {
    fn root(&self) -> anyhow::Result<MerkleRoot> {
        // # Dev-note: If `ChainConfig` got a new field, maybe we need to hash it too.
        // Avoid using the `..` in the code below. Use `_` instead if you don't need to hash
        // the field. Explicit fields help to prevent a bug of missing fields in the hash.
        let ChainConfig {
            chain_name,
            block_gas_limit,
            // Skip the `initial_state` bec
            initial_state: _,
            consensus_parameters,
            consensus,
        } = self;

        // TODO: Hash settlement configuration when it will be available.
        let config_hash = *Hasher::default()
            .chain(chain_name.as_bytes())
            .chain(block_gas_limit.to_be_bytes())
            .chain(consensus_parameters.root()?)
            .chain(consensus.root()?)
            .finalize();

        Ok(config_hash)
    }
}

impl GenesisCommitment for ConsensusParameters {
    fn root(&self) -> anyhow::Result<MerkleRoot> {
        // TODO: Define hash algorithm for `ConsensusParameters`
        let bytes = postcard::to_allocvec(&self).map_err(anyhow::Error::msg)?;
        let params_hash = Hasher::default().chain(bytes).finalize();

        Ok(params_hash.into())
    }
}

impl GenesisCommitment for GasCosts {
    fn root(&self) -> anyhow::Result<MerkleRoot> {
        // TODO: Define hash algorithm for `GasCosts`
        let bytes = postcard::to_allocvec(&self).map_err(anyhow::Error::msg)?;
        let hash = Hasher::default().chain(bytes).finalize();

        Ok(hash.into())
    }
}

impl GenesisCommitment for ConsensusConfig {
    fn root(&self) -> anyhow::Result<MerkleRoot> {
        // TODO: Define hash algorithm for `ConsensusConfig`
        let bytes = postcard::to_allocvec(&self).map_err(anyhow::Error::msg)?;
        let hash = Hasher::default().chain(bytes).finalize();

        Ok(hash.into())
    }
}
