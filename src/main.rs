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
    mrkl_root: String,
    time: u64,
    bits: u32,
    nonce: u32,
    n_tx: u32,
    size: u32,
    block_index: u64,
    main_chain: bool,
    height: u32,
    received_time: u64,
    #[serde(deserialize_with = "deserialize_string_or_struct")]
    relayed_by: String,
    tx: Vec<Value>,
}

fn deserialize_string_or_struct<'de, D>(deserializer: D) -> std::result::Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => Ok(s),
        _ => Ok("".to_string()),
    }
}

#[derive(Deserialize, Debug)]
struct Transaction {
    from: String,
    to: String,
    value: String,
    gas: String,
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
                    println!("Block Hash: {}", block.hash);
                    println!("Version: {}", block.ver);
                    println!("Previous Block: {}", block.prev_block);
                    println!("Merkle Root: {}", block.mrkl_root);
                    println!("Timestamp: {}", block.time);
                    println!("Bits: {}", block.bits);
                    println!("Nonce: {}", block.nonce);
                    println!("Number of Transactions: {}", block.n_tx);
                    println!("Block Size: {} bytes", block.size);
                    println!("Block Index: {}", block.block_index);
                    println!("Is Main Chain: {}", block.main_chain);
                    println!("Block Height: {}", block.height);
                    println!("Received Time: {}", block.received_time);
                    println!("Relayed By: {}", block.relayed_by);
                    println!("Number of transactions: {}", block.tx.len());
                }
                Err(e) => println!("Failed to fetch block details: {}", e),
            }
        }
        "transaction" => {
            match fetch_transaction(&args[2]) {
                Ok(transaction) => {
                    println!("From: {}", transaction.from);
                    println!("To: {}", transaction.to);
                    println!("Value: {}", transaction.value);
                    println!("Gas: {}", transaction.gas);
                }
                Err(e) => println!("Failed to fetch transaction details: {}", e),
            }
        }
        _ => println!("Invalid command. Use 'blockhash' or 'transaction'."),
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
        let text = response.text()?;
        println!("Raw response: {}", text);
        
        let block: Block = serde_json::from_str(&text)?;
        Ok(block)
    } else {
        Err(Box::new(response.error_for_status().unwrap_err()))
    }
}

fn fetch_transaction(id: &str) -> Result<Transaction> {
    let url = format!("https://api.blockchain.com/transaction/{}", id);
    let client = Client::new();

    let response = client.get(&url)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36")
        .header(ACCEPT, "application/json")
        .send()?;

    if response.status().is_success() {
        Ok(response.json()?)
    } else {
        Err(Box::new(response.error_for_status().unwrap_err()))
    }
}