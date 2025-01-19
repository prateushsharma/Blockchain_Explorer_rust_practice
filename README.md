# Blockchain_Explorer_rust_practice

A simple Rust-based command-line tool for exploring Bitcoin blockchain data.

## Quick Start

1. Clone and build:
```bash
git clone https://github.com/yourusername/Blockchain_Explorer_rust_practice.git
cd Blockchain_Explorer_rust_practice
cargo build
```

2. Run with test block:
```bash
cargo run blockhash 00000000000000000000898a8085aebfed62b49824616b0fa15bd10a82f0115b
```

## Features

- Fetch block details using block hash
- Display block information including:
  - Block header data
  - Transaction summaries
  - Block statistics
- Error handling for invalid inputs

## Dependencies

```toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking", "json"] }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"
```

## Usage

Basic command syntax:
```bash
cargo run blockhash <BLOCK_HASH>
```

Example output includes:
- Block hash
- Version
- Previous block hash
- Merkle root
- Timestamp
- Block size
- Number of transactions
- First 5 transactions summary

## Note

This is a practice project using the [Blockchain.info API](https://blockchain.info/) for educational purposes.
