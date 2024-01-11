use fuel_core::{service::FuelService, chain_config::{StateConfig, ChainConfig}};

pub fn snapshot(fuel_service: &FuelService) -> anyhow::Result<ChainConfig> {

    let chain_config: String = "local_testnet".to_string();

    let config: ChainConfig = chain_config.parse()?;
    let state_conf = StateConfig::generate_state_config(fuel_service.shared.database.clone())?;

    let chain_conf = ChainConfig {
        initial_state: Some(state_conf),
        ..config
    };

    Ok(chain_conf)
}

pub trait SnapshotStringify {
    fn stringify(self) -> anyhow::Result<String>;
}

impl SnapshotStringify for ChainConfig {
    fn stringify(self) -> anyhow::Result<String> {
        let stringified = serde_json::to_string_pretty(&self)?;

        Ok(stringified)    
    }
}