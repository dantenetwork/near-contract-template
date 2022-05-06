use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::json;
use near_sdk::{env, log, near_bindgen, AccountId, Gas, PanicOnDefault, Promise};
use std::collections::{HashMap, HashSet};

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(tag = "type", crate = "near_sdk::serde")]
pub struct GreetingData {
    from_chain: String,
    title: String,
    content: String,
    date: String,
}

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Content {
    pub contract: String,
    pub action: String,
    pub data: String,
}

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(tag = "type", crate = "near_sdk::serde")]
pub struct DstContract {
    contract_address: String,
    action_name: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Greeting {
    owner_id: AccountId,
    cross_chain_contract_id: AccountId,
    greeting_data: UnorderedMap<String, GreetingData>,
    destination_contract: UnorderedMap<String, DstContract>,
    permitted_contract: UnorderedMap<String, HashMap<String, HashSet<String>>>,
}

#[near_bindgen]
impl Greeting {
    #[init]
    pub fn new(owner_id: AccountId, cross_chain_contract_id: AccountId) -> Self {
        Self {
            owner_id,
            cross_chain_contract_id,
            greeting_data: UnorderedMap::new(b"g"),
            destination_contract: UnorderedMap::new(b"o"),
            permitted_contract: UnorderedMap::new(b"p"),
        }
    }

    /**
     * Send greeting info to other chains
     * @param to_chain - to chain name
     * @param title - greeting title
     * @param title - greeting content
     * @param title - greeting date
     */
    pub fn send_greeting(
        &self,
        to_chain: String,
        title: String,
        content: String,
        date: String,
    ) -> Promise {
        let greeting_action_data = json!({
            "greeting": [to_chain, title, content, date]
        })
        .to_string();
        // log!("greeting_action_data: {}", greeting_action_data);
        Promise::new(self.cross_chain_contract_id.clone()).function_call(
            "send_message".to_string(),
            json!({
                "to_chain": to_chain,
                "content": Content{
                    contract: self.destination_contract.get(&to_chain).unwrap().contract_address,
                    action: self.destination_contract.get(&to_chain).unwrap().action_name,
                    data: greeting_action_data,
                }
            })
            .to_string()
            .into_bytes(),
            0,
            Gas(5_000_000_000_000),
        )
    }

    pub fn receive_greeting(&mut self, greeting: Vec<String>) -> bool {
        assert_eq!(
            env::predecessor_account_id(),
            self.cross_chain_contract_id,
            "only call by {}",
            self.cross_chain_contract_id
        );
        let data = GreetingData {
            from_chain: greeting[0].clone(),
            title: greeting[1].clone(),
            content: greeting[2].clone(),
            date: greeting[3].clone(),
        };
        self.greeting_data.insert(&greeting[0], &data);
        return true;
    }

    pub fn register_dst_contract(
        &mut self,
        chain_name: String,
        contract_address: String,
        action_name: String,
    ) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id.to_string(),
            "Unauthorize"
        );
        self.destination_contract.insert(
            &chain_name,
            &DstContract {
                contract_address,
                action_name,
            },
        );
    }

    ///////////////////////////////////////////////
    ///    Receive messages from other chains   ///
    ///////////////////////////////////////////////

    /**
     * Authorize contracts of other chains to call the action of this contract
     * @param chain_name - from chain name
     * @param sender - sender of cross chain message
     * @param action_name - action name which allowed to be invoked
     */
    pub fn register_permitted_contract(
        &mut self,
        chain_name: String,
        sender: String,
        action_name: String,
    ) {
        assert_eq!(self.owner_id, env::predecessor_account_id(), "Unauthorize");
        if let Some(contracts) = self.permitted_contract.get(&chain_name) {
            // if let action_name
            if let Some(actions) = contracts.get(&sender) {
                assert!(actions.contains(&action_name), "Already exist");
                actions.insert(action_name);
                contracts.insert(sender, *actions);
            } else {
                let hs = HashSet::new();
                hs.insert(action_name);
                contracts.insert(sender, hs);
            }
            self.permitted_contract.insert(&chain_name, &contracts);
        } else {
            let hs = HashSet::new();
            hs.insert(action_name);
            let hm = HashMap::new();
            hm.insert(sender, hs);
            self.permitted_contract.insert(&chain_name, &hm);
        }
    }

    pub fn get(&self, from_chain: String) -> Option<GreetingData> {
        self.greeting_data.get(&from_chain)
    }

    pub fn set_owner_id(&mut self, owner_id: AccountId) {
        assert_eq!(self.owner_id, env::predecessor_account_id(), "Unauthorize");
        self.owner_id = owner_id;
    }

    pub fn clear_data(&mut self, chains: Vec<String>) {
        assert_eq!(self.owner_id, env::predecessor_account_id(), "Unauthorize");
        if chains.len() == 0 {
            self.greeting_data.clear();
        } else {
            for chain in chains {
                self.greeting_data.remove(&chain);
            }
        }
    }
}
