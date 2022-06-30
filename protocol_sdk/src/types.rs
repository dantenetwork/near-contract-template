use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
// use near_sdk::serde_json::{self, json, Value};
// use crate::payload;

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Content {
    pub contract: String,
    pub action: String,
    pub data: Vec<u8>,
}

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct SQoS {
    pub t: u8,
    pub v: Option<String>,
}

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Session {
    pub id: u64,
    pub callback: Option<String>,
}

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Message {
    pub from_chain: String,
    pub to_chain: String,
    pub sender: String,
    pub signer: String,
    pub sqos: Vec<SQoS>,
    pub content: Content,
    pub session: Session,
}

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Context {
    pub id: u64,
    pub from_chain: String,
    pub sender: String,
    pub signer: String,
    pub contract_id: String,
    pub action: String,
    pub sqos: Vec<SQoS>,
    pub session: Session,
}

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(tag = "type", crate = "near_sdk::serde")]
pub struct DstContract {
    pub contract_address: String,
    pub action_name: String,
}

// #[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
// #[serde(crate = "near_sdk::serde")]
// pub struct Field(Vec<Value>);

// impl Field {
//     pub fn new(vec: Vec<Value>) -> Field {
//         Field(vec)
//     }
// }
#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Value {
    String(String),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(U128),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    VecString(Vec<String>),
    VecUint8(Vec<u8>),
    VecUint16(Vec<u16>),
    VecUint32(Vec<u32>),
    VecUint64(Vec<u64>),
    VecUint128(Vec<U128>),
    VecInt8(Vec<i8>),
    VecInt16(Vec<i16>),
    VecInt32(Vec<i32>),
    VecInt64(Vec<i64>),
}

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MessageItem {
    pub n: String,
    pub v: Value,
}

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Payload {
    pub items: Vec<MessageItem>,
}

impl Payload {
    pub fn new() -> Payload {
        Payload { items: Vec::new() }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.try_to_vec().unwrap()
    }

    pub fn decode(data: &Vec<u8>) -> Payload {
        Payload::try_from_slice(data.as_slice()).unwrap()
    }

    pub fn push_item(&mut self, n: String, v: Value) {
        let item = MessageItem { n, v };
        if !self.items.contains(&item) {
            self.items.push(item)
        }
    }

    pub fn get_item(&self, n: String) -> Option<Value> {
        for item in self.items.iter() {
            if item.n == n {
                return Some(item.v.clone());
            }
        }
        None
    }
}

impl Message {
    pub fn to_hash(&self) -> String {
        let message_serialized: Vec<u8> = self.try_to_vec().unwrap();
        hex::encode(env::sha256(message_serialized.as_slice()))
    }
}

impl Value {
    pub fn get_value<T: ValueType>(&self) -> Option<T::Type> {
        T::get_value::<T>(self)
    }
}

pub trait ValueType {
    type Type;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type>;
}

impl ValueType for String {
    type Type = String;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::String(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }
}

impl ValueType for u8 {
    type Type = u8;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::Uint8(val) = *type_value {
            Some(val)
        } else {
            None
        }
    }
}

impl ValueType for u16 {
    type Type = u16;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::Uint16(val) = *type_value {
            Some(val)
        } else {
            None
        }
    }
}

impl ValueType for u32 {
    type Type = u32;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::Uint32(val) = *type_value {
            Some(val)
        } else {
            None
        }
    }
}

impl ValueType for u64 {
    type Type = u64;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::Uint64(val) = *type_value {
            Some(val)
        } else {
            None
        }
    }
}

impl ValueType for U128 {
    type Type = U128;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::Uint128(val) = *type_value {
            Some(val)
        } else {
            None
        }
    }
}

impl ValueType for i8 {
    type Type = i8;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::Int8(val) = *type_value {
            Some(val)
        } else {
            None
        }
    }
}

impl ValueType for i16 {
    type Type = i16;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::Int16(val) = *type_value {
            Some(val)
        } else {
            None
        }
    }
}

impl ValueType for i32 {
    type Type = i32;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::Int32(val) = *type_value {
            Some(val)
        } else {
            None
        }
    }
}

impl ValueType for i64 {
    type Type = i64;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::Int64(val) = *type_value {
            Some(val)
        } else {
            None
        }
    }
}

// impl ValueType for i128 {
//     type Type = i128;
//     fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
//         if let Value::Int128(val) = *type_value {
//             Some(val)
//         } else {
//             None
//         }
//     }
// }

impl ValueType for Vec<String> {
    type Type = Vec<String>;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecString(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }
}

impl ValueType for Vec<u8> {
    type Type = Vec<u8>;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecUint8(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }
}

impl ValueType for Vec<u16> {
    type Type = Vec<u16>;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecUint16(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }
}

impl ValueType for Vec<u32> {
    type Type = Vec<u32>;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecUint32(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }
}

impl ValueType for Vec<u64> {
    type Type = Vec<u64>;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecUint64(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }
}

impl ValueType for Vec<U128> {
    type Type = Vec<U128>;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecUint128(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }
}

impl ValueType for Vec<i8> {
    type Type = Vec<i8>;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecInt8(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }
}

impl ValueType for Vec<i16> {
    type Type = Vec<i16>;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecInt16(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }
}

impl ValueType for Vec<i32> {
    type Type = Vec<i32>;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecInt32(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }
}
impl ValueType for Vec<i64> {
    type Type = Vec<i64>;
    fn get_value<Type>(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecInt64(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }
}
