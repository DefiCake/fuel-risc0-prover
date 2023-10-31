use derive_more::{
    Add,
    AsRef,
    Deref,
    Display,
    From,
    FromStr,
    Into,
    LowerHex,
    Rem,
    Sub,
    UpperHex,
};

use fuel_types::Bytes32;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    FromStr,
    From,
    Into,
    LowerHex,
    UpperHex,
    Display,
    AsRef,
)]
pub struct BlockId(Bytes32);

impl BlockId {
    /// Converts the hash into a message having the same bytes.
    pub fn into_message(self) -> fuel_crypto::Message {
        fuel_crypto::Message::from_bytes(*self.0)
    }

    /// Converts the hash into a message having the same bytes.
    pub fn as_message(&self) -> &fuel_crypto::Message {
        fuel_crypto::Message::from_bytes_ref(&self.0)
    }

    /// Represents `BlockId` as slice of bytes.
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl AsRef<[u8]> for BlockId {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

/// Block height of the data availability layer
#[derive(
    Sub,
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
    PartialOrd,
    Eq,
    Add,
    Ord,
    Display,
    Into,
    From,
    Rem,
    Deref,
    Hash,
    serde::Serialize,
    serde::Deserialize
)]
#[rem(forward)]
pub struct DaBlockHeight(pub u64);

impl From<DaBlockHeight> for Vec<u8> {
    fn from(height: DaBlockHeight) -> Self {
        height.0.to_be_bytes().to_vec()
    }
}

impl From<usize> for DaBlockHeight {
    fn from(n: usize) -> Self {
        DaBlockHeight(n as u64)
    }
}

impl core::ops::Add<u64> for DaBlockHeight {
    type Output = Self;

    fn add(self, other: u64) -> Self::Output {
        Self::from(self.0 + other)
    }
}

impl DaBlockHeight {
    /// Convert to array of big endian bytes
    pub fn to_bytes(self) -> [u8; 8] {
        self.0.to_be_bytes()
    }

    /// Convert to usize
    pub fn to_usize(self) -> usize {
        self.0 as usize
    }

    /// Convert to usize
    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }

    /// Convert to u64
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}