use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Content {
    pub contract: String,
    pub action: String,
    pub data: String,
}

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Session {
    pub res_type: u8,
    pub id: Option<u64>,
}

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(tag = "type", crate = "near_sdk::serde")]
pub struct DstContract {
    pub contract_address: String,
    pub action_name: String,
}
