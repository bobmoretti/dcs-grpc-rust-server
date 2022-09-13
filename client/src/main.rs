// use std::io::{self, BufRead}
pub mod client;
use client::DcsGrpcClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rpc_client = DcsGrpcClient::new();
    let indication = rpc_client.list_indication(4)?;
    println!("Indication:\n{}", indication);
    println!("Aircraft name: {}", rpc_client.get_aircraft_name()?);
    Ok(())
}

// // Now you have a runtime that can drive a future to completion.
// // Anywhere you wanna do that in normal/synchronous code:
// runtime.block_on(some_async_fn_or_block);
