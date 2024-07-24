use std::env;
use alloy::{
    providers::{Provider, ProviderBuilder, WsConnect},
    sol
};
use eyre::Result;
use colored::Colorize; 
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url: String = env::var("WS_ALTITUDE_ENDPOINT_NOLECHE")
        .expect("RPC_URL not set");
    let ws = WsConnect::new(&rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await?;
    
    println!("connecting to {}", rpc_url.black().on_cyan());


    // subscribe to new blocks 
        let sub = provider.subscribe_blocks().await?;

    // Wait and take the next 4 blocks.
    let mut stream = sub.into_stream().take(4);

    println!("Awaiting blocks...");

    // Take the stream and print the block number upon receiving a new block.
    let handle = tokio::spawn(async move {
        while let Some(block) = stream.next().await {
            println!(
                "Latest block number: {}",
                block.header.number.expect("Failed to get block number")
            );
        }
    });

    handle.await?;


    Ok(())
}
