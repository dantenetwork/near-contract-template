use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault,
    PromiseOrValue, PromiseResult,
};
use protocol_sdk::{Content, Context, OmniChain, Payload, Value};

const GAS_FOR_CALLBACK: Gas = Gas(5_000_000_000_000);

const NO_DEPOSIT: Balance = 0;

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ComputeTask {
    pub nums: Vec<u32>,
    pub result: Option<u32>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Computation {
    omni_chain: OmniChain,
    compute_task: UnorderedMap<(String, u128), ComputeTask>,
}

#[ext_contract(ext_self)]
pub trait MyContract {
    fn callback(&mut self, to_chain: String, nums: Vec<u32>) -> U128;
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    DestinationContract,
    PermittedContract,
    Result,
}

#[near_bindgen]
impl Computation {
    #[init]
    pub fn new(owner_id: AccountId, omni_chain_contract_id: AccountId) -> Self {
        Self {
            omni_chain: OmniChain::new(
                owner_id,
                StorageKey::DestinationContract,
                StorageKey::PermittedContract,
                omni_chain_contract_id,
            ),
            compute_task: UnorderedMap::new(StorageKey::Result),
        }
    }

    pub fn send_compute_task(&mut self, to_chain: String, nums: Vec<u32>) -> PromiseOrValue<U128> {
        let mut payload = Payload::new();
        payload.push_item("nums".to_string(), Value::VecUint32(nums.clone()));
        let action_name = "receive_compute_task".to_string();
        let dst_contract = self
            .omni_chain
            .destination_contract
            .get(&to_chain.clone())
            .expect("to chain not register");
        let contract = dst_contract
            .get(&action_name)
            .expect("contract not register");
        let content = Content {
            contract: contract.contract_address.clone(),
            action: contract.action_name.clone(),
            data: payload,
        };
        let callback = "receive_compute_result".as_bytes().to_vec();
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

    pub fn receive_compute_task(&self, payload: Payload, context: Context) {
        assert_eq!(
            self.omni_chain.omni_chain_contract_id,
            env::predecessor_account_id(),
            "Processs by cross chain contract"
        );
        self.omni_chain.assert_register_permitted_contract(
            &context.from_chain,
            &context.sender,
            &context.action,
        );

        let item = payload.get_item("nums".to_string()).unwrap();
        let nums = item.get_value::<Vec<u32>>().unwrap();

        let mut sum: u32 = 0;
        for num in nums {
            sum += num;
        }

        let mut payload = Payload::new();
        payload.push_item("result".to_string(), Value::Uint32(sum));
        let content = Content {
            contract: context.sender,
            action: context.session.callback.unwrap(),
            data: payload,
        };
        self.omni_chain
            .send_response_message(context.from_chain, content, context.session.id);
    }

    pub fn receive_compute_result(&mut self, payload: Payload, context: Context) {
        assert_eq!(
            self.omni_chain.omni_chain_contract_id,
            env::predecessor_account_id(),
            "Processs by cross chain contract."
        );
        let item = payload.get_item("result".to_string()).unwrap();
        let result = item.get_value::<u32>().unwrap();
        let session = context.session;
        let id = session.id.0;
        let key = (context.from_chain, id);
        self.compute_task.get(&key).as_mut().and_then(|task| {
            task.result = Some(result);
            self.compute_task.insert(&key, task)
        });
    }

    pub fn get_compute_task(&self, to_chain: String, id: U128) -> Option<ComputeTask> {
        self.compute_task.get(&(to_chain, id.0))
    }

    #[private]
    pub fn callback(&mut self, to_chain: String, nums: Vec<u32>) -> U128 {
        // let mut session_id: u64 = 0;
        match env::promise_result(0) {
            PromiseResult::Successful(result) => {
                let session_id = near_sdk::serde_json::from_slice::<U128>(&result)
                    .expect("unwrap session id failed");
                self.compute_task.insert(
                    &(to_chain, session_id.0),
                    &ComputeTask { nums, result: None },
                );
                session_id
            }
            _ => env::panic_str("call omi-chain failed"),
        }
    }

    pub fn get_dst_contract(&self, chain: String, action_name: String) -> (Vec<u8>, Vec<u8>) {
        let dst_contract = self.omni_chain.destination_contract.get(&chain).unwrap();
        // let action_name = "receive_"
        let contract = dst_contract.get(&action_name).unwrap();
        (
            contract.contract_address.clone(),
            contract.action_name.clone(),
        )
    }
    // UnorderedMap<(String, String), Vec<String>>,
    pub fn get_permitted_contract(&self) -> Vec<((String, Vec<u8>), Vec<String>)> {
        self.omni_chain
            .permitted_contract
            .iter()
            .map(|res| res)
            .collect()
    }

    pub fn clear_compute_task(&mut self) {
        self.compute_task.clear();
    }
}

protocol_sdk::impl_omni_chain_register!(Computation, omni_chain);
