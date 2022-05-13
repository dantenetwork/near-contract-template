use crate::types::{Content, DstContract, Session};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{ext_contract, AccountId, Balance, Gas, IntoStorageKey, Promise};

const GAS_FOR_SENT_MESSAGE: Gas = Gas(5_000_000_000_000);

const NO_DEPOSIT: Balance = 0;

#[ext_contract(ext_cross_contract)]
pub trait CrossChainContract {
    fn send_message(&mut self, to_chain: String, content: Content, session: Option<Session>)
        -> u64;
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct CrossChain {
    pub cross_chain_contract_id: AccountId,
    pub destination_contract: UnorderedMap<String, DstContract>,
}

impl CrossChain {
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
        self.destination_contract.insert(
            &chain_name,
            &DstContract {
                contract_address,
                action_name,
            },
        );
    }
}
