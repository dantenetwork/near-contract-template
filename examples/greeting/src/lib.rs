use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, PromiseOrValue};
use protocol_sdk::{Content, Context, OmniChain, Payload, Value};

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct GreetingData {
    from_chain: String,
    title: String,
    content: String,
    date: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Greeting {
    omni_chain: OmniChain,
    greeting_data: UnorderedMap<(String, u128), GreetingData>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    DestinationContract,
    PermittedContract,
    GreetingData,
}

#[near_bindgen]
impl Greeting {
    #[init]
    pub fn new(owner_id: AccountId, omni_chain_contract_id: AccountId) -> Self {
        Self {
            omni_chain: OmniChain::new(
                owner_id,
                StorageKey::DestinationContract,
                StorageKey::PermittedContract,
                omni_chain_contract_id,
            ),
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
    pub fn send_greeting(
        &self,
        to_chain: String,
        title: String,
        content: String,
        date: String,
    ) -> PromiseOrValue<U128> {
        let mut payload = Payload::new();
        let greeting_data = Value::VecString(vec!["NEARTEST".to_string(), title, content, date]);
        payload.push_item("greeting".to_string(), greeting_data);
        let action_name = "send_greeting".to_string();
        let dst_contract = self
            .omni_chain
            .destination_contract
            .get(&to_chain)
            .expect("to chain not register");
        let contract = dst_contract
            .get(&action_name)
            .expect("contract not register");
        let content = Content {
            contract: contract.contract_address.clone(),
            action: contract.action_name.clone(),
            data: payload,
        };
        self.omni_chain.call_cross(to_chain, content).into()
    }

    pub fn receive_greeting(&mut self, payload: Payload, context: Context) {
        assert_eq!(
            env::predecessor_account_id(),
            self.omni_chain.omni_chain_contract_id,
            "Processs by cross chain contract"
        );
        self.omni_chain.assert_register_permitted_contract(
            &context.from_chain,
            &context.sender,
            &context.action,
        );
        let item = payload.get_item("greeting".to_string()).unwrap();
        let greeting = item.get_value::<Vec<String>>().unwrap();
        let data = GreetingData {
            from_chain: greeting[0].clone(),
            title: greeting[1].clone(),
            content: greeting[2].clone(),
            date: greeting[3].clone(),
        };
        self.greeting_data
            .insert(&(greeting[0].clone(), context.id.0), &data);
    }

    pub fn get_greeting(&self, from_chain: String, id: U128) -> Option<GreetingData> {
        self.greeting_data.get(&(from_chain, id.0))
    }

    pub fn clear_greeting_data(&mut self) {
        self.greeting_data.clear();
    }
}

protocol_sdk::impl_omni_chain_register!(Greeting, omni_chain);
