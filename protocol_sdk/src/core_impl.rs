use crate::types::{Content, DstContract, Session};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::{env, ext_contract, AccountId, Balance, Gas, IntoStorageKey, Promise};
use std::collections::HashMap;

const GAS_FOR_SENT_MESSAGE: Gas = Gas(5_000_000_000_000);

const NO_DEPOSIT: Balance = 0;

#[ext_contract(ext_cross_contract)]
pub trait OmniChainContract {
    fn send_message(
        &mut self,
        to_chain: String,
        content: Content,
        session: Option<Session>,
    ) -> u128;
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct OmniChain {
    pub owner_id: AccountId,
    pub omni_chain_contract_id: AccountId,
    pub destination_contract: UnorderedMap<String, HashMap<String, DstContract>>,
    pub permitted_contract: UnorderedMap<(String, Vec<u8>), Vec<String>>,
}

impl OmniChain {
    pub fn new<S, T>(
        owner_id: AccountId,
        destination_contract_prefix: S,
        permitted_contract_prefix: T,
        omni_chain_contract_id: AccountId,
    ) -> Self
    where
        S: IntoStorageKey,
        T: IntoStorageKey,
    {
        let this = Self {
            owner_id,
            omni_chain_contract_id,
            destination_contract: UnorderedMap::new(destination_contract_prefix),
            permitted_contract: UnorderedMap::new(permitted_contract_prefix),
        };
        this
    }

    pub fn internal_call_omni_chain(
        &self,
        to_chain: String,
        content: Content,
        session: Option<Session>,
    ) -> Promise {
        ext_cross_contract::send_message(
            to_chain,
            content,
            session,
            self.omni_chain_contract_id.clone(),
            NO_DEPOSIT,
            GAS_FOR_SENT_MESSAGE,
        )
    }

    pub fn call_cross(&self, to_chain: String, content: Content) -> Promise {
        self.internal_call_omni_chain(to_chain, content, None)
    }

    pub fn call_cross_with_session(
        &self,
        to_chain: String,
        content: Content,
        callback: Vec<u8>,
    ) -> Promise {
        self.internal_call_omni_chain(
            to_chain,
            content,
            Some(Session {
                id: U128(0),
                session_type: 2,
                callback: Some(callback),
                commitment: None,
                answer: None,
            }),
        )
    }

    pub fn send_response_message(&self, to_chain: String, content: Content, id: U128) {
        self.internal_call_omni_chain(
            to_chain,
            content,
            Some(Session {
                id,
                session_type: 1,
                callback: None,
                commitment: None,
                answer: None,
            }),
        );
    }

    pub fn register_dst_contract(
        &mut self,
        chain_name: String,
        action_name: String,
        contract_address: String,
        contract_action_name: String,
    ) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Unauthorize");
        let contract_address = hex::decode(contract_address).unwrap();
        let contract_action_name = hex::decode(contract_action_name).unwrap();
        match self.destination_contract.get(&chain_name) {
            Some(mut map) => {
                // if !map.contains_key(&action_name) {
                //     map.insert(
                //         action_name,
                //         DstContract {
                //             contract_address,
                //             action_name: contract_action_name,
                //         },
                //     );
                // } else {
                //     env::panic_str("Already contains");
                // }
                map.insert(
                    action_name,
                    DstContract {
                        contract_address,
                        action_name: contract_action_name,
                    },
                );
                self.destination_contract.insert(&chain_name, &map);
            }
            _ => {
                let mut ms = HashMap::new();
                ms.insert(
                    action_name,
                    DstContract {
                        contract_address,
                        action_name: contract_action_name,
                    },
                );
                self.destination_contract.insert(&chain_name, &ms);
            }
        }
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
        // assert_eq!(self.owner_id, env::predecessor_account_id(), "Unauthorize");
        let sender = hex::decode(sender).unwrap();
        let key = (chain_name, sender);
        let mut actions = Vec::new();
        if let Some(acts) = self.permitted_contract.get(&key) {
            assert!(!actions.contains(&action_name), "Already exist");
            actions.extend(acts.into_iter());
        }
        actions.push(action_name);
        self.permitted_contract.insert(&key, &actions);
    }

    pub fn assert_register_permitted_contract(
        &self,
        chain_name: &String,
        sender: &Vec<u8>,
        action: &String,
    ) {
        let key = (chain_name.clone(), sender.clone());
        let actions = self.permitted_contract.get(&key).unwrap_or(Vec::new());
        assert!(actions.contains(action), "{} not register", action);
    }
}
