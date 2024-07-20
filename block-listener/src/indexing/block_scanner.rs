use alloy::{providers::Provider, providers::ProviderBuilder, primitives::U64};
use tokio::time::sleep;
use colored::*;
use eyre::Result;
use std::time::Duration;

pub async fn loop_blocks() -> Result<()> {
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);
    let mut last_block = U64::ZERO;

    'main:loop {
        match provider.get_block_number().await {
            Ok(block) => {
                let block = U64::from(block);
                if block > last_block {
                    println!("\n\n{} {}", "New block".green(), block.to_string().blue());
                    last_block = block;
                    let block_details = provider.get_block_by_number(block.try_into().unwrap(), true).await;
                    let txs = block_details.unwrap().unwrap().transactions;
                    println!("{:?} new transactions found \n", txs.hashes().count());
                    sleep(Duration::from_secs(1)).await;

                    let vec = txs.hashes().collect::<Vec<_>>();
                    for (i, tx) in vec.iter().enumerate() {
                        if i >= 100 { break; } // TODO: remove this for full scan
                        
                        if let Ok(Some(tx_details)) = provider.get_transaction_by_hash(**tx).await {
                                
                            let transaction_type = tx_details.transaction_type.unwrap();


                            println!("Transaction: {}", tx.to_string().blue());
                            // cf https://docs.rs/alloy/latest/alloy/rpc/types/struct.Transaction.html
                            match transaction_type {
                                // EIP-4844 
                                3 => {
                                    println!("Type: {}", "proto-danksharding".green());
                                },
                                // EIP-1559
                                2 => {
                                    println!("Type: {}", "EIP-1559".red().bold());
                                },
                                // EIP-2930
                                1 => {
                                    println!("Type: {}", "Acces List ".magenta());
                                },
                                _  => {
                                    println!("Type: {}", "Legacy Transfer".green());
                                }
                            }

                            println!("Type: {}", transaction_type);
                            println!("From: {}", tx_details.from.to_string().cyan());
                            println!("To: {}", tx_details.to.map_or("Contract Creation".to_string(), |to| to.to_string()).cyan());
                            println!("Value: {} ETH", (f64::from(tx_details.value)  / 1e18).to_string().magenta());
                            
                            println!("{}", "--------------------------------------------------------------------------------".on_blue());
                            println!("\n");
                        }
                    }
                    println!("Just reviewed block {} \nand {} transactions", block.to_string().blue(), txs.hashes().count().to_string().on_magenta());
                }
            }
            Err(e) => {
                eprintln!("{}{}", "Error fetching block number: ".red(), e.to_string().blue());
            }
        }
        //sleep(Duration::from_secs(1)).await;
    }
}    
