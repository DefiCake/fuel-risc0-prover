mod types;

use methods::{ PROVER_ELF, PROVER_ID };
use clap::Parser;
use std::{ io::Write, time::Duration };

use anyhow::Result;
use bonsai_ethereum_relay::{ Relayer, EthersClientConfig };
use bonsai_sdk::{ alpha_async::{ get_client_from_parts, put_image }, alpha::SdkErr };
use ethers::{ providers::{ Provider, Http, Middleware }, types::H160 };
use types::JsonDeployment;

#[derive(Parser)]
struct Args {
  rpc_url: Option<String>,
  private_key: Option<String>,
}

const DEFAULT_PRIVATE_KEY: &str = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

#[tokio::main]
async fn main() -> Result<()> {
  let image_id = serialize_image_id()?;

  let args: Args = Args::parse();
  let rpc_url: &String = &args.rpc_url.unwrap_or_else(|| { String::from("http://localhost:8545") });
  let private_key: &String = &args.private_key.unwrap_or_else(|| { String::from(DEFAULT_PRIVATE_KEY) });
  let provider = Provider::<Http>::try_from(rpc_url.clone())?;
  let eth_chain_id = provider.get_chainid().await.expect("Could not get chain ID").as_u64();

  let bonsai_api_url = "http://localhost:8081".to_string();
  let bonsai_api_key = "".to_string();

  let provider = Provider::<Http>::try_from(rpc_url)?;
  let relay_contract_address: H160 = get_local_bonsai_relay_deployment(provider).await?;

  let relayer = Relayer {
    rest_api: true,
    dev_mode: true,
    rest_api_port: "8080".to_string(),
    bonsai_api_url: bonsai_api_url.clone(),
    bonsai_api_key: bonsai_api_key.clone(),
    relay_contract_address,
  };

  let client_config = EthersClientConfig::new(
    "ws://localhost:8545".to_string(), // bullshit here, hardhat 's ws implementation is buggy as hell...
    eth_chain_id,
    private_key.clone().try_into()?,
    10,
    Duration::from_secs(10)
  );
  let server_handle = tokio::spawn(relayer.run(client_config));

  // HACK: Wait 1 second to give local Bonsai a chance to start.
  std::thread::sleep(std::time::Duration::from_secs(1));

  let bonsai_client = get_client_from_parts(bonsai_api_url.to_string(), bonsai_api_key.to_string()).await?;

  (match put_image(bonsai_client.clone(), image_id.clone(), PROVER_ELF.clone().to_vec()).await {
    Ok(()) | Err(SdkErr::ImageIdExists) => Ok::<_, anyhow::Error>(()),
    Err(err) => Err(err.into()),
  })?;

  // Wait for the server to exit.
  let _ = server_handle.await;

  Ok(())
}

fn serialize_image_id() -> anyhow::Result<String> {
  let image_id = hex::encode(Vec::from(bytemuck::cast::<[u32; 8], [u8; 32]>(PROVER_ID)));

  let mut file = std::fs::File::create("res/IMAGE_ID")?;
  file.write_all(image_id.as_bytes())?;
  file.flush()?;
  drop(file);

  Ok(image_id)
}

async fn get_local_bonsai_relay_deployment(provider: Provider<Http>) -> anyhow::Result<H160> {
  match provider.resolve_name("relay").await {
    Ok(addr) => Ok(addr),
    Err(_) => {
      let json = std::fs
        ::read_to_string("deployments/local/BonsaiTestRelay.json".to_string())
        .expect("Could not load BonsaiTestRelay.json");

      let deployment: JsonDeployment = serde_json::from_str(json.as_str())?;

      let encoded = hex::decode(deployment.address.strip_prefix("0x").unwrap())?;
      Ok(H160::from_slice(encoded.as_slice()))
    }
  }
}
