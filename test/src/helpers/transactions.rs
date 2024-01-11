use fuels::{accounts::{provider::Provider, wallet::{WalletUnlocked, Wallet}, Account}, types::{transaction_builders::{ScriptTransactionBuilder, BuildableTransaction}, transaction::TxPolicies}};

use super::constants::{default_alice, default_bob};


/**
 * On this file
 */

 

pub async fn alice_sends_bob_100(
    provider: &Provider, 
    alice: Option<WalletUnlocked>, 
    bob: Option<Wallet>,
    commit: bool,
) -> anyhow::Result<()> {
    let mut alice = alice.unwrap_or_else(default_alice);
    alice.set_provider(provider.clone());

    let bob = bob.unwrap_or_else(|| Wallet::from_address(default_bob().address().clone(), Some(provider.clone())));

    let amount = 100u64;
    let asset_id = Default::default();
    let tx_policies: TxPolicies = Default::default();
    let network_info = provider.network_info().await?;
    let inputs = alice.get_asset_inputs_for_amount(asset_id, amount).await?;
    let outputs = alice.get_asset_outputs_for_amount(bob.address(), asset_id, amount);

    let mut tx_builder =
        ScriptTransactionBuilder::prepare_transfer(inputs, outputs, tx_policies, network_info);
    
    alice.add_witnessses(&mut tx_builder);
    alice.adjust_for_fee(&mut tx_builder, amount).await?;

    let tx = tx_builder.build(provider).await?;

    if commit {
        provider.send_transaction_and_await_commit(tx).await?;
    } else {
        provider.send_transaction(tx).await?;
    }

    Ok(())
}