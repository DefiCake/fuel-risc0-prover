use fuel_core::{
    database::Database,
    chain_config::{ ChainConfig, StateConfig, CoinConfig },
    service::{Config as FuelServiceConfig, FuelService} ,
    types::
      fuel_types::AssetId
      
    ,
};
use fuel_crypto::fuel_types::{Address, Bytes32};

use fuels::{
    prelude:: WalletUnlocked ,
    accounts::ViewOnlyAccount,
};

const DEFAULT_MNEMONIC_PHRASE: &str = "test test test test test test test test test test test junk";
const N_ACCOUNTS: u8 = 20;

pub async fn bootstrap1() -> anyhow::Result<FuelService> {
    let accounts: Vec<WalletUnlocked> = vec![0..N_ACCOUNTS]
        .iter()
        .enumerate()
        .map(|(index,_)| {
            WalletUnlocked::new_from_mnemonic_phrase_with_path(
                DEFAULT_MNEMONIC_PHRASE, 
                None, 
                format!("m/44'/60'/0'/0/{}", index).as_str()
            ).expect("Could not instantiate account")
        })
        .collect();

    let coins: Vec<CoinConfig> = accounts
        .clone()
        .iter()
        .enumerate()
        .map(|(index, account)| {
            let asset_id: AssetId = Default::default();
            let amount = 10_000_000;
            
            let mut vec_tx_id = vec![0u8; 32];
            vec_tx_id[31] = index as u8;
            let tx_id_slice: &[u8; 32] = vec_tx_id.as_slice().try_into().expect("asd");            
            let tx_id = Bytes32::from_bytes_ref(tx_id_slice).clone();
            
            CoinConfig {
                tx_id: Some(tx_id),
                output_index: Some(0),
                tx_pointer_block_height: Some(0.into()),
                tx_pointer_tx_idx: Some(0),
                maturity: Some(0.into()),
                owner: Address::new(*account.address().clone().hash),
                amount,
                asset_id,
            }
        }
    ).collect();

    let fuel_service_config = FuelServiceConfig {
      chain_conf: ChainConfig {
        initial_state: Some(StateConfig {
          coins: Some(coins),
          height: Some((0).into()),
          ..Default::default()
        }),
        ..ChainConfig::local_testnet()
      },
      ..FuelServiceConfig::local_node()
    };
  
    let database = Database::in_memory();
  
    let srv = FuelService::from_database(database.clone(), fuel_service_config.clone()).await.unwrap();
    srv.await_relayer_synced().await.unwrap();
  
    // dbg!(database.ids_of_latest_block());
  
    // snapshot(srv.shared.database.clone(), "snapshot_a.json".into()).expect("Failed to do first snapshot");
  
    // let provider = Provider::connect(srv.bound_address.to_string()).await.unwrap();
  
    // let block_a = srv.shared.database.get_current_block().unwrap().unwrap();
  
    // let sender_wallet = WalletUnlocked::new_from_private_key(alice_secret, Some(provider.clone()));
    // let receiver_wallet = FuelsViewWallet::from_address(bob.into(), None);
    
  
    // let mut inputs = vec![];
    // let i = sender_wallet.get_asset_inputs_for_amount(asset_id_alice, alice_value / 2).await.unwrap();
    // inputs.extend(i);
  
    // let mut outputs = vec![];
    // let o = sender_wallet.get_asset_outputs_for_amount(receiver_wallet.address(), asset_id_alice, alice_value / 2);
    // outputs.extend(o);
  
    // let network_info = provider.network_info().await.unwrap();
  
    // let mut tb: ScriptTransactionBuilder = 
    //   ScriptTransactionBuilder::prepare_transfer(inputs, outputs, Default::default(), network_info.clone());
    // sender_wallet.sign_transaction(&mut tb);
    
    // let tx = tb.build(&provider).await.expect("Could not build tx");
  
    // let unwrapped_tx: Script = tx.clone().into();
    // serde_json::to_string_pretty(&unwrapped_tx)
    //   .and_then(|stringified| {
    //     let path = "transaction.json";
    //     OpenOptions::new()
    //       .create(true)
    //       .write(true)
    //       .truncate(true)
    //       .open(&path)
    //       .expect("Could not open path");
    //     write(&path, stringified).expect("Failed to write to path");
  
    //     Ok(())
    //   }).expect("Failed to serialize transaction");
  
    // let tx_id = provider.send_transaction_and_await_commit(tx).await.unwrap();
  
    // loop {
    //   let receipts = provider.tx_status(&tx_id).await;
  
    //   if receipts.is_ok() {
    //     break;
    //   }
    // }
  
    // pb.finish_with_message("Waiting for receipt... Finished");
  
    // let receipts = provider.tx_status(&tx_id).await.unwrap().take_receipts();
  
    // dbg!(receipts);
  
    // // let alice_balance = provider.get_asset_balance(&alice.clone().into(), Default::default()).await.unwrap();
    // // let bob_balance = provider.get_asset_balance(&bob.clone().into(), Default::default()).await.unwrap();
    // // dbg!(alice_balance);
    // // dbg!(bob_balance);
  
    // let block_b = srv.shared.database.get_current_block().unwrap().unwrap();
    
    // // This does not get me enough information to rebuild the block and block transition...
    // to_json_file(&block_a, "block_a.json".to_string()).expect("Failed block_a json write");
    // to_json_file(&block_b, "block_b.json".to_string()).expect("Failed block_b write");
  
    // block_a.to_bincode_file("block_a.bincode".to_string()).expect("Failed block_a bincode write");
    // block_b.to_bincode_file("block_b.bincode".to_string()).expect("Failed block_a bincode write");
    // snapshot(srv.shared.database.clone(), "snapshot_b.json".into()).expect("Failed to do second snapshot");
  
    // let read_block_b: Block<Bytes32> = BinFileSerde::from_bincode_file("block_b.bincode".to_string()).expect(
    //   "Failed to roundtrip block_b.bincode"
    // );
  
    // assert_eq!(read_block_b, block_b.clone().into_owned());
    // dbg!(block_b.header());
    // dbg!(block_b.header().hash());

    anyhow::Ok(srv)
  }
  