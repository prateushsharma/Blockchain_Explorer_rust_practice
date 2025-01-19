use std::env;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Block {
    number: String,
    timestamp: String,
    transactions: Vec<String>,
    miner: String,
}

#[derive(Deserialize, Debug)]
struct Transaction {
    from: String,
    to: String,
    value: String,
    gas: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run <block/transaction> <id>");
        return Ok(());
    }

    match args[1].as_str() {
        "block" => {
            if let Ok(block) = fetch_block(&args[2]).await {
                println!("Block Number: {}", block.number);
                println!("Timestamp: {}", block.timestamp);
                println!("Transactions: {:?}", block.transactions);
                println!("Miner: {}", block.miner);
            } else {
                println!("Failed to fetch block details.");
            }
        }
        "transaction" => {
            if let Ok(transaction) = fetch_transaction(&args[2]).await {
                println!("From: {}", transaction.from);
                println!("To: {}", transaction.to);
                println!("Value: {}", transaction.value);
                println!("Gas: {}", transaction.gas);
            } else {
                println!("Failed to fetch transaction details.");
            }
        }
        _ => println!("Invalid command. Use 'block' or 'transaction'."),
    }

    Ok(())
}

async fn fetch_block(id: &str) -> Result<Block, Error> {
    let url = format!("https://api.blockchain.com/block/{}", id);
    let response = reqwest::get(&url).await?;
    let block = response.json::<Block>().await?;
    Ok(block)
}

async fn fetch_transaction(id: &str) -> Result<Transaction, Error> {
    let url = format!("https://api.blockchain.com/transaction/{}", id);
    let response = reqwest::get(&url).await?;
    let transaction = response.json::<Transaction>().await?;
    Ok(transaction)
}
