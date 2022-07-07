# NEAR contract template

NEAR protocol SDK makes it easy for NEAR developers in the NEAR Ecosystem to use Dante Network to interact with other chains, such as POLKADOT, Ethereum, Avalanche, Flow, etc.


## Install dependency

```sh
# For compile contract
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup target add wasm32-unknown-unknown
# For interact with the NEAR blockchain.
npm install -g near-cli
```

Click [use this templete](https://github.com/dantenetwork/near-contract-template/generate) to start your multi-ecosystem dApp.

* Create your project in [project](./project). You can find more details there.
* Refer to the [greeting](https://github.com/dantenetwork/near-contract-template/tree/main/contracts/greeting) or [computing](https://github.com/dantenetwork/near-contract-template/tree/main/contracts/computing) case to build your project.

Or you  can use this SDK as a library by adding

```toml
protocol_sdk = { path = "<local path of protocol_sdk>/protocol_sdk"}
```

## Library

### [call_cross](https://github.com/dantenetwork/near-contract-template/blob/develop/protocol_sdk/src/core_impl.rs#L61)

The function `call_cross` sends a cross-chain message, and without return.

Example is shown below, or you can refer it in the example [greeting](https://github.com/dantenetwork/near-contract-template/blob/develop/examples/greeting/src/lib.rs#L52).

```rust
pub fn send_greeting(&self, to_chain: String, title: String, content: String, date: String) {
    let mut payload = Payload::new();
    let greeting_data = Value::VecString(vec!["NEARTEST".to_string(), title, content, date]);
    payload.push_item("greeting".to_string(), greeting_data);
    ....
    let content = Content {
        contract: contract.contract_address.clone(),
        action: contract.action_name.clone(),
        data: payload,
    };
    self.omni_chain.call_cross(to_chain, content);
}
```

### [call_cross_with_session](https://github.com/dantenetwork/near-contract-template/blob/develop/protocol_sdk/src/core_impl.rs#L65)

The function `call_cross_with_session` sends a cross-chain message, and returns the session id, which is equal the sent message id recorded in the cross-chain contract. Later a callback in the application contract will be called.

Example is shown below, or you can refer it in the example [computing](https://github.com/dantenetwork/near-contract-template/blob/develop/examples/computing/src/lib.rs#L55).

```rust
pub fn send_compute_task(&mut self, to_chain: String, nums: Vec<u32>) -> PromiseOrValue<u64> {
    let mut payload = Payload::new();
    payload.push_item("nums".to_string(), Value::VecUint32(nums.clone()));
    ...
    let callback = "receive_compute_result".to_string();
    self.omni_chain
        .call_cross_with_session(to_chain.clone(), content, callback)
        .then(ext_self::callback(
            to_chain,
            nums,
            env::current_account_id(),
            NO_DEPOSIT,
            GAS_FOR_CALLBACK,
        ))
        .into()
}
```

### [send_response_message](https://github.com/dantenetwork/near-contract-template/blob/develop/protocol_sdk/src/core_impl.rs#L81)

The function `cross_chain_respond` responds a cross-chain request, and returns the session id recorded in the cross-chain contract.

Example is shown below, or you can refer it in the example [computing](https://github.com/dantenetwork/near-contract-template/blob/develop/examples/computing/src/lib.rs#L85).

```rust
pub fn receive_compute_task(&self, payload: Payload, context: Context) {
    ...
    let content = Content {
        contract: context.sender,
        action: context.session.callback.unwrap(),
        data: payload,
    };
    self.omni_chain
        .send_response_message(context.from_chain, content, context.session.id);
}
```
### Examples
#### Compile smart contract

```sh
cd examples/greeting
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
mkdir -p ./res && cp target/wasm32-unknown-unknown/release/greeting.wasm ./res/
```

#### Deploy smart contract to NEAR testnet

```sh
cd examples/greeting
near deploy $CONTRACT_ID --wasmFile res/greeting.wasm --accountId $CONTRACT_ID
```
