# Stellar Lockbox

A secure and efficient smart contract for managing time-locked assets on the Stellar blockchain.

## Overview

Stellar Lockbox is a smart contract that enables users to lock XLM or other Stellar assets for a specified duration. The locked assets can only be withdrawn by the designated recipient after the lock period has expired.

## Features

- 🔒 Time-based asset locking
- ⏳ Configurable lock periods
- 🔄 Support for any Stellar asset
- 🔍 Transparent on-chain verification
- 🛡️ Secure and non-custodial

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/install)
- [Stellar Testnet account](https://laboratory.stellar.org/#account-creator?network=test)
- [Freighter Wallet](https://www.freighter.app/) (for testnet interactions)

## Getting Started

### Clone the Repository

```bash
git clone https://github.com/yourusername/stellar-lockbox.git
cd stellar-lockbox
```

### Build the Contract

```bash
cd contracts/lockbox
cargo build --target wasm32-unknown-unknown --release
```

### Test the Contract

```bash
cargo test
```

## Deployment

### Deploy to Testnet

1. Make sure you have your testnet account configured with Freighter
2. Deploy the contract:

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/lockbox.wasm \
  --source your-wallet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"
```

## Usage

### Initialize a New Lockbox

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source your-wallet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- initialize \
  --owner <OWNER_ACCOUNT> \
  --token <TOKEN_ADDRESS> \
  --amount <AMOUNT> \
  --unlock_at <UNIX_TIMESTAMP>
```

### Withdraw from Lockbox

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source your-wallet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- withdraw
```

## Security

This contract has not been audited. Use at your own risk.

## License

MIT