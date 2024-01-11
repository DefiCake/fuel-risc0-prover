use fuels::accounts::wallet::WalletUnlocked;

pub const DEFAULT_MNEMONIC_PHRASE: &str = "test test test test test test test test test test test junk";
pub const N_ACCOUNTS: u8 = 20;

pub fn default_alice() -> WalletUnlocked {
    WalletUnlocked::new_from_mnemonic_phrase_with_path(DEFAULT_MNEMONIC_PHRASE, None, "m/44'/60'/0'/0/0").unwrap()
}

pub fn default_bob() -> WalletUnlocked {
    WalletUnlocked::new_from_mnemonic_phrase_with_path(DEFAULT_MNEMONIC_PHRASE, None, "m/44'/60'/0'/0/1").unwrap()
}