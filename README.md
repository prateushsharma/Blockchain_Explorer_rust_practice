# Blockchain Explorer

A Rust-based command-line tool for exploring Bitcoin blockchain data. This application fetches and displays detailed information about Bitcoin blocks and their transactions using the blockchain.info API.

## Features

- Fetch detailed block information using block hash
- Display comprehensive block details including:
  - Block hash, version, and height
  - Previous block reference
  - Merkle root
  - Timestamp
  - Mining difficulty (bits)
  - Nonce
  - Block size and transaction count
  - Block index and chain status
- Transaction summaries for the first 5 transactions in each block
- Proper error handling for API requests and data parsing
- User-friendly command-line interface

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)
- Internet connection to access the blockchain.info API

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/blockchain_explorer.git
cd blockchain_explorer
```

2. Build the project:
```bash
cargo build
```

## Usage

To fetch information about a specific block, use the following command:

```bash
cargo run blockhash <BLOCK_HASH>
```

Example:
```bash
cargo run blockhash 00000000000000000000898a8085aebfed62b49824616b0fa15bd10a82f0115b
```

This will display:
- Block Information (hash, version, previous block, merkle root, etc.)
- Block Statistics (size, number of transactions)
- Transaction Summary (details of the first 5 transactions)

## Project Structure

```
src/
  ├── main.rs          # Main application logic
  └── lib.rs           # Library code and structs
Cargo.toml            # Project dependencies and metadata
README.md            # Project documentation
```

## Dependencies

- reqwest: HTTP client for API requests
- serde: Serialization/deserialization framework
- serde_json: JSON parsing support
- tokio: Async runtime for Rust

## Error Handling

The application handles various error cases including:
- Invalid block hashes
- Network connection issues
- API response parsing errors
- Missing or malformed data fields

## API Reference

This project uses the blockchain.info API:
- Block Information: `https://blockchain.info/rawblock/{block_hash}`

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature-name`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin feature/your-feature-name`
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Future Enhancements

Planned features for future releases:
- Transaction value summaries
- Block reward calculations
- Transaction filtering options
- Human-readable timestamp formatting
- Detailed transaction analysis
- Support for multiple blockchain APIs
- Cache system for frequently accessed data
- Export functionality to various formats (CSV, JSON)

## Acknowledgments

- blockchain.info for providing the API
- The Rust community for excellent documentation and crate ecosystem
- Contributors who help improve this project

## Support

If you encounter any issues or have questions, please file an issue on the GitHub repository.

## Version History

- 1.0.0 (2024-01-19)
  - Initial release
  - Basic block information retrieval
  - Transaction summary display
  - Error handling implementation
