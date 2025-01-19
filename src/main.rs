use std::env;
use reqwest::{blocking::Client, header::{USER_AGENT, ACCEPT}};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::error::Error as StdError;

type Result<T> = std::result::Result<T, Box<dyn StdError>>;

#[derive(Deserialize, Debug)]
struct Block {
    hash: String,
    ver: u32,
    prev_block: String,
    #[serde(rename = "mrkl_root")]
    merkle_root: String,
    time: u64,
    bits: u32,
    nonce: u32,
    n_tx: u32,
    size: u32,
    block_index: u64,
    main_chain: bool,
    height: u32,
    #[serde(default)]  // Make received_time optional
    received_time: u64,
    #[serde(default)]  // Make relayed_by optional
    relayed_by: String,
    tx: Vec<Transaction>,
}

#[derive(Deserialize, Debug)]
struct Transaction {
    #[serde(default)]
    hash: String,
    #[serde(default)]
    ver: u32,
    #[serde(default)]
    inputs: Vec<Input>,
    #[serde(default)]
    out: Vec<Output>,
}

#[derive(Deserialize, Debug)]
struct Input {
    sequence: u64,
    #[serde(default)]
    witness: String,
    script: String,
    index: u32,
    prev_out: Option<PrevOut>,
}

#[derive(Deserialize, Debug)]
struct Output {
    #[serde(rename = "type")]
    output_type: u32,
    spent: bool,
    value: u64,
    #[serde(default)]
    spending_outpoints: Vec<SpendingOutpoint>,
    n: u32,
    tx_index: u64,
    script: String,
    addr: Option<String>,
}

#[derive(Deserialize, Debug)]
struct PrevOut {
    #[serde(rename = "type")]
    output_type: u32,
    spent: bool,
    value: u64,
    spending_outpoints: Vec<SpendingOutpoint>,
    n: u32,
    tx_index: u64,
    script: String,
    addr: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SpendingOutpoint {
    tx_index: u64,
    n: u32,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run <block/transaction> <id>");
        return Ok(());
    }

    match args[1].as_str() {
        "blockhash" => {
            match fetch_block(&args[2]) {
                Ok(block) => {
                    println!("\nBlock Information:");
                    println!("------------------");
                    println!("Block Hash: {}", block.hash);
                    println!("Version: {}", block.ver);
                    println!("Previous Block: {}", block.prev_block);
                    println!("Merkle Root: {}", block.merkle_root);
                    println!("Timestamp: {}", block.time);
                    println!("Bits: {}", block.bits);
                    println!("Nonce: {}", block.nonce);
                    println!("Number of Transactions: {}", block.n_tx);
                    println!("Block Size: {} bytes", block.size);
                    println!("Block Index: {}", block.block_index);
                    println!("Is Main Chain: {}", block.main_chain);
                    println!("Block Height: {}", block.height);
                    
                    println!("\nTransactions Summary:");
                    println!("--------------------");
                    println!("Total Transactions: {}", block.tx.len());
                    
                    // Print first few transactions as example
                    for (i, tx) in block.tx.iter().take(5).enumerate() {
                        println!("\nTransaction #{}", i + 1);
                        println!("Hash: {}", tx.hash);
                        println!("Version: {}", tx.ver);
                        println!("Input Count: {}", tx.inputs.len());
                        println!("Output Count: {}", tx.out.len());
                    }
                    
                    if block.tx.len() > 5 {
                        println!("\n... and {} more transactions", block.tx.len() - 5);
                    }
                }
                Err(e) => println!("Failed to fetch block details: {}", e),
            }
        }
        _ => println!("Invalid command. Use 'blockhash'."),
    }

    Ok(())
}

fn fetch_block(block_hash: &str) -> Result<Block> {
    let url = format!("https://blockchain.info/rawblock/{}", block_hash);
    let client = Client::new();

    let response = client.get(&url)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36")
        .header(ACCEPT, "application/json")
        .send()?;

    if response.status().is_success() {
        let block = response.json::<Block>()?;
        Ok(block)
    } else {
        Err(Box::new(response.error_for_status().unwrap_err()))
    }
}