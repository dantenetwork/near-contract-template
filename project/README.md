# Start your project here

## Clone `near-contract-template` to your local
```bash
git clone https://github.com/dantenetwork/near-contract-template
```

## Create a New project

```bash
cargo new --lib <your project name>
```

## Add `protocol_sdk` library into `Cargo.toml`, like following:

```toml
protocol_sdk = { path = "./near-contract-template/protocol_sdk"}
```

This will be published on `crates.io` later.

## Add `near` library into `Cargo.toml`, like following:

```toml
near-sdk = "4.0.0-pre.7"
```


## More details
* [Official tutorial](https://docs.near.org/docs/develop/contracts/rust/intro)