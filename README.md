# NEAR contract template

## Introduction

This is a high level SDK for NEAR developers to build their dApps based on Danta protocol stack, with the help of which dApps built on Near can be extended to Omnichain. 

### Install

```sh
# For compile contract
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup target add wasm32-unknown-unknown
# For interact with the NEAR blockchain.
npm install -g near-cli
```

### Build contract
* Click [use this templete](https://github.com/dantenetwork/near-contract-template/generate) to create your own project.
* Refer to the [greeting](https://github.com/dantenetwork/near-contract-template/tree/main/contracts/greeting) or [computing](https://github.com/dantenetwork/near-contract-template/tree/main/contracts/computing) case to build your project.

### Example
#### Compile smart contract

```sh
cd contracts/greeting
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
mkdir -p ./res && cp target/wasm32-unknown-unknown/release/greeting.wasm ./res/
```

#### Deploy smart contract to NEAR testnet

```sh
cd contracts/greeting
near deploy $CONTRACT_ID --wasmFile res/greeting.wasm --accountId $CONTRACT_ID
```
