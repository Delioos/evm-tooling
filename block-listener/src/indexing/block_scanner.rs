use alloy::{providers::Provider, providers::ProviderBuilder, primitives::U64};
use tokio::time::sleep;
use colored::*;
use eyre::Result;
use std::time::Duration;
// the block_explorer file is in the same directory as this file

pub async fn loop_blocks() -> Result<()> {
    // Set up the HTTP transport which is consumed by the RPC client.
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);
    let mut last_block = U64::ZERO;
    loop {
        match provider.get_block_number().await {
            Ok(block) => {
                let block = U64::from(block);
                if block > last_block {
                    println!("\n\n{} {}", "New block".green(), block.to_string().blue());
                    last_block = block;
                    // Get the block details
                    let block_details = provider.get_block_by_number(block.try_into().unwrap(), true).await;

                    // extract the transactions 
                    let txs = block_details.unwrap().unwrap().transactions;
                    println!("{:?} new transactions found \n", txs.hashes().count());

                    // sleep a bit
                    sleep(Duration::from_secs(1)).await;
                    txs.hashes().for_each(|tx| {
                        println!("{:?}", tx); // conditionnal coloring on sell or buy 
                    });


                }
            }
            Err(e) => {
                eprintln!("{}{}", "Error fetching block number: ".red(), e.to_string().blue());
            }
        }
        // Add a small delay to avoid hammering the API
        sleep(Duration::from_secs(1)).await;
    }
}
