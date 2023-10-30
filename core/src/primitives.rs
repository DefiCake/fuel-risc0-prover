use fuel_types::Bytes32;

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