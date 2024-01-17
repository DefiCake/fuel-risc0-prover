use std::ops::Deref;

use fuel_core::{database::Database, types::{blockchain::block::Block, services::p2p::Transactions}};

pub fn get_current_block_with_txs(database: &Database) -> anyhow::Result<Block> {
    let block = database.get_current_block()?.unwrap();
    let start = block.header().height().deref().clone();
    let end = start + 1;
    let initial_transaction_set = 
        match database.get_transactions_on_blocks(start..end)? {
            Some(value) => {
                if value.len() == 1 {
                    value.first().unwrap().clone()
                } else {
                    Transactions(vec![])
                }
            }
            None => Transactions(vec![])
        };
    let initial_block_with_txs = 
        Block::try_from_executed(block.header().clone(), initial_transaction_set.0).unwrap();

    Ok(initial_block_with_txs)
}