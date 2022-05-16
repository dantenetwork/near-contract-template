use crate::types::{Content, DstContract, Session};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, ext_contract, AccountId, Balance, Gas, IntoStorageKey, Promise};
use std::collections::{HashMap, HashSet};

const GAS_FOR_SENT_MESSAGE: Gas = Gas(5_000_000_000_000);

const NO_DEPOSIT: Balance = 0;

#[ext_contract(ext_cross_contract)]
pub trait CrossChainContract {
    fn send_message(&mut self, to_chain: String, content: Content, session: Option<Session>)
        -> u64;
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct CrossChain {
    pub owner_id: AccountId,
    pub cross_chain_contract_id: AccountId,
    pub destination_contract: UnorderedMap<String, DstContract>,
    pub permitted_contract: UnorderedMap<String, HashMap<String, HashSet<String>>>,
}

impl CrossChain {
    pub fn new<S, T>(
        owner_id: AccountId,
        destination_contract_prefix: S,
        permitted_contract_prefix: T,
        cross_chain_contract_id: AccountId,
    ) -> Self
    where
        S: IntoStorageKey,
        T: IntoStorageKey,
    {
        let this = Self {
            owner_id,
            cross_chain_contract_id,
            destination_contract: UnorderedMap::new(destination_contract_prefix),
            permitted_contract: UnorderedMap::new(permitted_contract_prefix),
        };
        this
    }

    pub fn internal_call_cross_chain(
        &self,
        to_chain: String,
        content: Content,
        session: Option<Session>,
    ) -> Promise {
        ext_cross_contract::send_message(
            to_chain,
            content,
            session,
            self.cross_chain_contract_id.clone(),
            NO_DEPOSIT,
            GAS_FOR_SENT_MESSAGE,
        )
    }

    pub fn call_cross(&self, to_chain: String, content: Content) {
        self.internal_call_cross_chain(to_chain, content, None);
    }

    pub fn call_cross_with_session(&self, to_chain: String, content: Content) -> Promise {
        self.internal_call_cross_chain(
            to_chain,
            content,
            Some(Session {
                res_type: 1,
                id: None,
            }),
        )
    }

    pub fn send_response_message(&self, to_chain: String, content: Content, id: u64) {
        self.internal_call_cross_chain(
            to_chain,
            content,
            Some(Session {
                res_type: 2,
                id: Some(id),
            }),
        );
    }

    pub fn register_dst_contract(
        &mut self,
        chain_name: String,
        contract_address: String,
        action_name: String,
    ) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Unauthorize");
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
        // assert_eq!(self.owner_id, env::predecessor_account_id(), "Unauthorize");
        if let Some(mut contracts) = self.permitted_contract.get(&chain_name) {
            // if let action_name
            if let Some(actions) = contracts.get_mut(&sender) {
                assert!(!actions.contains(&action_name), "Already exist");
                actions.insert(action_name);
                // contracts.insert(sender, *actions);
            } else {
                let mut hs = HashSet::new();
                hs.insert(action_name);
                contracts.insert(sender, hs);
            }
            self.permitted_contract.insert(&chain_name, &contracts);
        } else {
            let mut hs = HashSet::new();
            hs.insert(action_name);
            let mut hm = HashMap::new();
            hm.insert(sender, hs);
            self.permitted_contract.insert(&chain_name, &hm);
        }
    }

    pub fn assert_register_permitted_contract(
        &self,
        chain_name: &String,
        sender: &String,
        action: &String,
    ) {
        let permitted_contract = self
            .permitted_contract
            .get(chain_name)
            .expect("Not register permitted contract");
        let actions = permitted_contract
            .get(sender)
            .expect("message sender not register");
        assert!(actions.contains(action), "{} not register", action);
    }
}
