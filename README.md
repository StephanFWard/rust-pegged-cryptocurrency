# rust-pegged-cryptocurrency
A simple pegged cryptocurrency using Rust and the Substrate blockchain framework

# Pegged Cryptocurrency Example with Substrate and Rust

This example demonstrates the implementation of a pegged cryptocurrency using the Substrate blockchain framework and Rust programming language.

## Overview

The pegged cryptocurrency allows users to mint new tokens by depositing an equivalent amount in the reserve and burn tokens to withdraw the corresponding amount from the reserve. The example is a basic demonstration and should be extended for a production environment.

## Getting Started

### Prerequisites

- Rust: [Install Rust](https://www.rust-lang.org/tools/install)
- Substrate: [Install Substrate](https://substrate.dev/docs/en/knowledgebase/getting-started/)

### Building and Running

1. **Clone the repository:**

   ```
   git clone https://github.com/StephanFWard/rust-pegged-cryptocurrency.git
   cd substrate-pegged-cryptocurrency

2. **Build the project:**
   ```
   cargo build --release

3. **Run the substrate node:**
   ```
   ./target/release/substrate-pegged-cryptocurrency --dev

4. **Interact with the node using the Substrate UI or Polkadot JS Apps.**
Mint new tokens:

   ```
   ./target/release/substrate-pegged-cryptocurrency mint <your-account-id> <amount>

5. **Burn tokens:**

   ```
   ./target/release/substrate-pegged-cryptocurrency burn <your-account-id> <amount>

6. **Run tests using:**

   ```
   cargo test

License
This project is licensed under general public use.
