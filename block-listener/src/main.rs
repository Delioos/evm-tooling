mod indexing;

use crate::indexing::block_scanner;

#[tokio::main]
async fn main() {
    block_scanner::loop_blocks().await.unwrap();
} 
