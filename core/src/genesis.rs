use fuel_crypto::Hasher;
use fuel_storage::MerkleRoot;
use fuel_types::Bytes32;

pub trait GenesisCommitment {
    /// Calculates the merkle root of the state of the entity.
    fn root(&self) -> anyhow::Result<MerkleRoot>;
}

/// The first block of the blockchain is a genesis block. It determines the initial state of the
/// network - contracts states, contracts balances, unspent coins, and messages. It also contains
/// the hash on the initial config of the network that defines the consensus rules for following
/// blocks.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Genesis {
    /// The chain config define what consensus type to use, what settlement layer to use,
    /// rules of block validity, etc.
    pub chain_config_hash: Bytes32,
    /// The Binary Merkle Tree root of all genesis coins.
    pub coins_root: Bytes32,
    /// The Binary Merkle Tree root of state, balances, contracts code hash of each contract.
    pub contracts_root: Bytes32,
    /// The Binary Merkle Tree root of all genesis messages.
    pub messages_root: Bytes32,
}


impl GenesisCommitment for Genesis {
    fn root(&self) -> anyhow::Result<MerkleRoot> {
        let genesis_hash = *Hasher::default()
            .chain(self.chain_config_hash)
            .chain(self.coins_root)
            .chain(self.contracts_root)
            .chain(self.messages_root)
            .finalize();

        Ok(genesis_hash)
    }
}
