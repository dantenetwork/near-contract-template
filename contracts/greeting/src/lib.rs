use dante_cross_chain_standards::{Content, CrossChain};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::json;
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(tag = "type", crate = "near_sdk::serde")]
pub struct GreetingData {
    from_chain: String,
    title: String,
    content: String,
    date: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Greeting {
    owner_id: AccountId,
    cross: CrossChain,
    greeting_data: UnorderedMap<String, GreetingData>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    CrossChain,
    GreetingData,
}

#[near_bindgen]
impl Greeting {
    #[init]
    pub fn new(owner_id: AccountId, cross_chain_contract_id: AccountId) -> Self {
        Self {
            owner_id,
            cross: CrossChain::new(StorageKey::CrossChain, cross_chain_contract_id),
            greeting_data: UnorderedMap::new(StorageKey::GreetingData),
        }
    }

    /**
     * Send greeting info to other chains
     * @param to_chain - to chain name
     * @param title - greeting title
     * @param title - greeting content
     * @param title - greeting date
     */
    pub fn send_greeting(&self, to_chain: String, title: String, content: String, date: String) {
        let greeting_action_data = json!({
            "greeting": ["NEAR".to_string(), title, content, date]
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
            data: greeting_action_data,
        };
        self.cross.call_cross(to_chain, content);
    }

    pub fn receive_greeting(&mut self, greeting: Vec<String>) {
        assert_eq!(
            env::predecessor_account_id(),
            self.cross.cross_chain_contract_id,
            "Processs by cross chain contract"
        );
        let data = GreetingData {
            from_chain: greeting[0].clone(),
            title: greeting[1].clone(),
            content: greeting[2].clone(),
            date: greeting[3].clone(),
        };
        self.greeting_data.insert(&greeting[0], &data);
    }

    pub fn get_greeting(&self, from_chain: String) -> Option<GreetingData> {
        self.greeting_data.get(&from_chain)
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
}
