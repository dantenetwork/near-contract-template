use dante_cross_chain_standards::{Content, Context, CrossChain};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json;
use near_sdk::{
    env, ext_contract, log, near_bindgen, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault,
    PromiseOrValue, PromiseResult,
};

const GAS_FOR_CALLBACK: Gas = Gas(5_000_000_000_000);

const NO_DEPOSIT: Balance = 0;

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(tag = "type", crate = "near_sdk::serde")]
pub struct ComputeTask {
    pub nums: Vec<u64>,
    pub result: Option<u64>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Computation {
    owner_id: AccountId,
    cross: CrossChain,
    compute_task: UnorderedMap<u64, ComputeTask>,
}

#[ext_contract(ext_self)]
pub trait MyContract {
    fn callback(&mut self, nums: Vec<u64>) -> u64;
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    CrossChain,
    Result,
}

#[near_bindgen]
impl Computation {
    #[init]
    pub fn new(owner_id: AccountId, cross_chain_contract_id: AccountId) -> Self {
        Self {
            owner_id,
            cross: CrossChain::new(StorageKey::CrossChain, cross_chain_contract_id),
            compute_task: UnorderedMap::new(StorageKey::Result),
        }
    }

    pub fn send_compute_task(&mut self, to_chain: String, nums: Vec<u64>) -> PromiseOrValue<u64> {
        let data = serde_json::json!({
            "_nums": nums,
        })
        .to_string();
        let content = Content {
            contract: self
                .cross
                .destination_contract
                .get(&to_chain)
                .unwrap()
                .contract_address,
            action: self
                .cross
                .destination_contract
                .get(&to_chain)
                .unwrap()
                .action_name,
            data,
        };
        self.cross
            .call_cross_with_session(to_chain, content)
            .then(ext_self::callback(
                nums,
                env::current_account_id(),
                NO_DEPOSIT,
                GAS_FOR_CALLBACK,
            ))
            .into()
        // self.cross
        //     .call_cross_with_session(to_chain, content)
        //     .and(ext_self::callback(
        //         nums,
        //         env::current_account_id(),
        //         NO_DEPOSIT,
        //         GAS_FOR_CALLBACK,
        //     ))
        //     .into()
    }

    pub fn receive_compute_task(&self, nums: Vec<u64>, context: Context) {
        assert_eq!(
            self.cross.cross_chain_contract_id,
            env::predecessor_account_id(),
            "Processs by cross chain contract"
        );
        let mut sum: u64 = 0;
        for num in nums {
            sum += num;
        }
        let data = serde_json::json!({
            "result": sum,
        })
        .to_string();
        let destination_contract = self
            .cross
            .destination_contract
            .get(&context.from_chain)
            .expect("not register");
        let content = Content {
            contract: destination_contract.contract_address,
            action: destination_contract.action_name,
            data,
        };
        self.cross
            .send_response_message(context.from_chain, content, context.id);
    }

    pub fn receive_compute_result(&mut self, result: u64, context: Context) {
        assert_eq!(
            self.cross.cross_chain_contract_id,
            env::predecessor_account_id(),
            "Processs by cross chain contract."
        );
        log!("Start receive_compute_result: {}", context.id);
        self.compute_task
            .get(&context.id)
            .as_mut()
            .and_then(|task| {
                task.result = Some(result);
                self.compute_task.insert(&context.id, task)
            });
    }

    pub fn get_compute_task(&self, id: u64) -> Option<ComputeTask> {
        self.compute_task.get(&id)
    }

    pub fn register_dst_contract(
        &mut self,
        chain_name: String,
        contract_address: String,
        action_name: String,
    ) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Unauthorize");
        self.cross
            .register_dst_contract(chain_name, contract_address, action_name);
    }

    #[private]
    pub fn callback(&mut self, nums: Vec<u64>) -> u64 {
        let mut session_id: u64 = 0;
        match env::promise_result(0) {
            PromiseResult::Successful(result) => {
                session_id = near_sdk::serde_json::from_slice::<u64>(&result)
                    .expect("unwrap session id failed");
                self.compute_task
                    .insert(&session_id, &ComputeTask { nums, result: None });
            }
            _ => env::panic_str("dsafd"),
        }
        session_id
    }
}
