# NEAR contract template

## Currently

**This is under construction!**

This repo contains a basic `greeting` smart contract integrated with DANTE cross-chain service.

The basic version developed in `near` is currently available.

## Coming soon

- More functions in `near` version;
- The high level SDK for `cross-chain contract call` is under developing, that will bring more convenient for developers.

## Usage

Dependencies:

* @dante-contracts

### Install

```sh
# For compile contract
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup target add wasm32-unknown-unknown
# For interact with the NEAR blockchain.
npm install -g near-cli
```

### Compile smart contract

```sh
cd contracts/greeting
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
mkdir -p ./res && cp target/wasm32-unknown-unknown/release/greeting.wasm ./res/
```

### Deploy smart contract to NEAR testnet

```sh
cd contracts/greeting
near deploy $CONTRACT_ID --wasmFile res/greeting.wasm --accountId $CONTRACT_ID
```

### Register greeting contract to DANTE cross-chain service

```sh
node register/registerToNear.js
```
