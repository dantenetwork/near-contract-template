use crate::types::{Content, DstContract};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde_json::json;
use near_sdk::{ext_contract, AccountId, IntoStorageKey, Promise, Gas};

const GAS_FOR_SENT_MESSAGE: Gas = Gas(5_000_000_000_000);


#[ext_contract(ext_cross_contract)]
pub trait FungibleTokenContract {
    fn send_message(&mut self, to_chain: String, content: Content);
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Standards {
    pub cross_chain_contract_id: AccountId,
    pub destination_contract: UnorderedMap<String, DstContract>,
}

impl Standards {
    pub fn new<S>(prefix: S, cross_chain_contract_id: AccountId) -> Self
    where
        S: IntoStorageKey,
    {
        let this = Self {
            cross_chain_contract_id,
            destination_contract: UnorderedMap::new(prefix),
        };
        this
    }
    
    pub fn send_message(
        &self,
        to_chain: String,
        content: Content
    ) {
        Promise::new(self.cross_chain_contract_id.clone()).function_call(
            "send_message".to_string(),
            json!({
                "to_chain": to_chain,
                "content": content,
            })
            .to_string()
            .into_bytes(),
            0,
            GAS_FOR_SENT_MESSAGE,
        );
    }
}
