# Solana Whirlpool Price Monitor

A Rust application that monitors and displays real-time prices of specified Solana Whirlpool pools in the terminal.$  

## Features

- Connects to the Solana mainnet via RPC
- Fetches and deserializes Whirlpool pool data
- Continuously updates and displays pool prices
- Highlights price changes in the terminal

## Prerequisites$  
- **Rust**: Ensure you have Rust installed. You can install it from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **Cargo**: Comes with Rust for managing dependencies and building the project.

## Setup$  
1. **Clone the Repository**  
    ```bash
    git clone https://github.com/yourusername/solana-whirlpool-price-monitor.git$  
    cd solana-whirlpool-price-monitor$  
    ```

2. **Add Dependencies**
    Ensure your `Cargo.toml` includes the following dependencies:
    ```toml
    [dependencies]
    solana-client = "1.14.0"       
    solana-program = "1.14.0"
    borsh = "0.10.3"
    termion = "1.5.6"
    uint = "0.9.0"
    ```

3. **Build the Project**
    ```bash
    cargo build --release
    ```

## Usage

Run the application using Cargo:
```bash
cargo run --release
```

The terminal will display the prices of the configured Solana Whirlpool pools, updating every 2 seconds.$

## Configuration
- **RPC URL**: The application connects to the Solana mainnet via `https://api.mainnet-beta.solana.com`. You can change this URL in the `main` function if needed.  
- **Pool Addresses**: Modify the `pool_addresses` vector in `main.rs` to include the public keys of the Whirlpool pools you want to monitor. 

## Dependencies$  
- [solana-client](https://crates.io/crates/solana-client)
- [solana-program](https://crates.io/crates/solana-program)
- [borsh](https://crates.io/crates/borsh)
- [termion](https://crates.io/crates/termion)
- [uint](https://crates.io/crates/uint)

