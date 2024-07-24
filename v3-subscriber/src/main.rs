use std::env;
use alloy::{
    providers::{Provider, ProviderBuilder, WsConnect},
    primitives::{keccak256, Address},
    rpc::types::{Filter, Log},
    sol_types::SolEvent,
    sol,
};
use eyre::Result;
use colored::Colorize;
use futures_util::StreamExt;

sol! {
    #[derive(Debug)]
    contract IUniswapV2Pair {
        event Swap(address indexed sender, uint amount0In, uint amount1In, uint amount0Out, uint amount1Out, address indexed to);
        event Sync(uint112 reserve0, uint112 reserve1);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url: String = env::var("WS_ALTITUDE_ENDPOINT_NOLECHE")
        .expect("RPC_URL not set");
    let ws = WsConnect::new(&rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await?;
    
    println!("connecting to {}", rpc_url.black().on_cyan());
    
    // Subscribe to Swap events
    let signature = keccak256("Swap(address,uint256,uint256,uint256,uint256,address)");
    let sub = provider.subscribe_logs(&Filter::new().event_signature(signature)).await?;
    
    println!("Awaiting logs... ");
    
    let mut stream = sub.into_stream().take(2);
    while let Some(log) = stream.next().await {
        println!("Raw log: {:#?}", log);
    }

    Ok(())
}
