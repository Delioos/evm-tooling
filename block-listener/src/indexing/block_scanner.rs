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
                    sleep(Duration::from_secs(1)).await;

                    // sleep a bit
                    /*
                    txs.hashes().for_each(|tx| {
                        println!("{:?}", tx); // conditionnal coloring on sell or buy 
                    });
                    */

                    // retrieve transactions details
                    let vec = txs.hashes().collect::<Vec<_>>();
                    let mut i = 0;
                    for tx in vec {
                        let tx_details = provider.get_transaction_by_hash(*tx).await;
                        println!("{:?}", tx_details);
                        i += 1;
                        match i {
                            1 => println!("{}", "--------------------------------------------------------------------------------".on_green()),
                            2 => println!("{}", "--------------------------------------------------------------------------------".on_blue()),
                            3 => println!("{}", "--------------------------------------------------------------------------------".on_magenta()),
                            4 => println!("{}", "--------------------------------------------------------------------------------".on_cyan()),
                            5 => println!("{}", "--------------------------------------------------------------------------------".on_yellow()),
                            6 => println!("{}", "--------------------------------------------------------------------------------".on_red()),
                            7 => println!("{}", "--------------------------------------------------------------------------------".on_black()),
                            8 => println!("{}", "--------------------------------------------------------------------------------".on_white()),
                            9 => println!("{}", "--------------------------------------------------------------------------------".on_green()),
                            10 => println!("{}", "--------------------------------------------------------------------------------".on_blue()),
                            _ => {}
                        }
                        println!("\n\n");
                        if i == 10 {
                            break;
                        }
                    }

                    println!("just reviewed block {} \n and {} transactions", block.to_string().blue(), txs.hashes().count().to_string().on_magenta());



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
