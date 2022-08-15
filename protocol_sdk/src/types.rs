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
    pub data: Payload,
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
pub struct Address(String, u8);

impl Address {
    pub fn new(address: String, address_type: u8) -> Self {
        Address(address, address_type)
    }

    pub fn get(&self) -> String {
        self.0.clone()
    }

    pub fn get_type(&self) -> u8 {
        self.1
    }
}

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
    Address(Address),
}

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MessageItem {
    pub name: String,
    pub value: Value,
}

#[derive(Clone, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Payload(Vec<MessageItem>);

impl Payload {
    pub fn new() -> Payload {
        Payload(Vec::new())
    }

    pub fn push_item(&mut self, name: String, value: Value) {
        let item = MessageItem { name, value };
        if !self.0.contains(&item) {
            self.0.push(item)
        }
    }

    pub fn get_item(&self, name: String) -> Option<Value> {
        for item in self.0.iter() {
            if item.name == name {
                return Some(item.value.clone());
            }
        }
        None
    }

    pub fn into_raw_data(&self) -> Vec<u8> {
        let mut raw_bytes: Vec<u8> = Vec::new();
        for item in self.0.iter() {
            let value_raw_bytes = match item.value.clone() {
                Value::String(value) => value.into_raw_data(),
                Value::Uint8(value) => value.into_raw_data(),
                Value::Uint16(value) => value.into_raw_data(),
                Value::Uint32(value) => value.into_raw_data(),
                Value::Uint64(value) => value.into_raw_data(),
                Value::Uint128(value) => value.into_raw_data(),
                Value::Int8(value) => value.into_raw_data(),
                Value::Int16(value) => value.into_raw_data(),
                Value::Int32(value) => value.into_raw_data(),
                Value::Int64(value) => value.into_raw_data(),
                Value::VecString(value) => value.into_raw_data(),
                Value::VecUint8(value) => value.into_raw_data(),
                Value::VecUint16(value) => value.into_raw_data(),
                Value::VecUint32(value) => value.into_raw_data(),
                Value::VecUint64(value) => value.into_raw_data(),
                Value::VecUint128(value) => value.into_raw_data(),
                Value::VecInt8(value) => value.into_raw_data(),
                Value::VecInt16(value) => value.into_raw_data(),
                Value::VecInt32(value) => value.into_raw_data(),
                Value::VecInt64(value) => value.into_raw_data(),
                Value::Address(value) => value.into_raw_data(),
            };
            raw_bytes.extend(value_raw_bytes);
        }
        raw_bytes
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
        T::get_value(self)
    }
}

pub trait ValueType {
    type Type;
    fn get_value(type_value: &Value) -> Option<Self::Type>;
    fn into_raw_data(&self) -> Vec<u8>;
}

impl ValueType for String {
    type Type = String;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::String(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }
    fn into_raw_data(&self) -> Vec<u8> {
        Vec::from(self.as_bytes())
    }
}

impl ValueType for u8 {
    type Type = u8;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::Uint8(val) = *type_value {
            Some(val)
        } else {
            None
        }
    }
    fn into_raw_data(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl ValueType for u16 {
    type Type = u16;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::Uint16(val) = *type_value {
            Some(val)
        } else {
            None
        }
    }
    fn into_raw_data(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl ValueType for u32 {
    type Type = u32;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::Uint32(val) = *type_value {
            Some(val)
        } else {
            None
        }
    }
    fn into_raw_data(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl ValueType for u64 {
    type Type = u64;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::Uint64(val) = *type_value {
            Some(val)
        } else {
            None
        }
    }
    fn into_raw_data(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl ValueType for U128 {
    type Type = U128;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::Uint128(val) = *type_value {
            Some(val)
        } else {
            None
        }
    }

    fn into_raw_data(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}

impl ValueType for i8 {
    type Type = i8;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::Int8(val) = *type_value {
            Some(val)
        } else {
            None
        }
    }
    fn into_raw_data(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl ValueType for i16 {
    type Type = i16;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::Int16(val) = *type_value {
            Some(val)
        } else {
            None
        }
    }
    fn into_raw_data(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl ValueType for i32 {
    type Type = i32;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::Int32(val) = *type_value {
            Some(val)
        } else {
            None
        }
    }
    fn into_raw_data(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl ValueType for i64 {
    type Type = i64;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::Int64(val) = *type_value {
            Some(val)
        } else {
            None
        }
    }
    fn into_raw_data(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

// impl ValueType for i128 {
//     type Type = i128;
//     fn get_value(type_value: &Value) -> Option<Self::Type> {
//         if let Value::Int128(val) = *type_value {
//             Some(val)
//         } else {
//             None
//         }
//     }
// }

impl ValueType for Vec<String> {
    type Type = Vec<String>;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecString(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }
    fn into_raw_data(&self) -> Vec<u8> {
        let mut raw_bytes = Vec::new();
        for value in self.into_iter() {
            raw_bytes.extend(value.as_bytes());
        }
        raw_bytes
    }
}

impl ValueType for Vec<u8> {
    type Type = Vec<u8>;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecUint8(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }
    fn into_raw_data(&self) -> Vec<u8> {
        let mut raw_bytes = Vec::new();
        for value in self.iter() {
            raw_bytes.push(*value);
        }
        raw_bytes
    }
}

impl ValueType for Vec<u16> {
    type Type = Vec<u16>;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecUint16(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }
    
    fn into_raw_data(&self) -> Vec<u8> {
        let mut raw_bytes = Vec::new();
        for value in self.iter() {
            raw_bytes.extend(value.to_be_bytes().to_vec());
        }
        raw_bytes
    }
}

impl ValueType for Vec<u32> {
    type Type = Vec<u32>;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecUint32(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn into_raw_data(&self) -> Vec<u8> {
        let mut raw_bytes = Vec::new();
        for value in self.iter() {
            raw_bytes.extend(value.to_be_bytes().to_vec());
        }
        raw_bytes
    }
}

impl ValueType for Vec<u64> {
    type Type = Vec<u64>;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecUint64(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn into_raw_data(&self) -> Vec<u8> {
        let mut raw_bytes = Vec::new();
        for value in self.iter() {
            raw_bytes.extend(value.to_be_bytes().to_vec());
        }
        raw_bytes
    }
}

impl ValueType for Vec<U128> {
    type Type = Vec<U128>;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecUint128(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn into_raw_data(&self) -> Vec<u8> {
        let mut raw_bytes = Vec::new();
        for value in self.iter() {
            raw_bytes.extend(value.0.to_be_bytes().to_vec());
        }
        raw_bytes
    }
}

impl ValueType for Vec<i8> {
    type Type = Vec<i8>;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecInt8(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn into_raw_data(&self) -> Vec<u8> {
        let mut raw_bytes = Vec::new();
        for value in self.iter() {
            raw_bytes.extend(value.to_be_bytes().to_vec());
        }
        raw_bytes
    }
}

impl ValueType for Vec<i16> {
    type Type = Vec<i16>;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecInt16(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn into_raw_data(&self) -> Vec<u8> {
        let mut raw_bytes = Vec::new();
        for value in self.iter() {
            raw_bytes.extend(value.to_be_bytes().to_vec());
        }
        raw_bytes
    }
}

impl ValueType for Vec<i32> {
    type Type = Vec<i32>;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecInt32(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn into_raw_data(&self) -> Vec<u8> {
        let mut raw_bytes = Vec::new();
        for value in self.iter() {
            raw_bytes.extend(value.to_be_bytes().to_vec());
        }
        raw_bytes
    }
}

impl ValueType for Vec<i64> {
    type Type = Vec<i64>;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::VecInt64(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn into_raw_data(&self) -> Vec<u8> {
        let mut raw_bytes = Vec::new();
        for value in self.iter() {
            raw_bytes.extend(value.to_be_bytes().to_vec());
        }
        raw_bytes
    }
}

impl ValueType for Address {
    type Type = Address;
    fn get_value(type_value: &Value) -> Option<Self::Type> {
        if let Value::Address(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }
    fn into_raw_data(&self) -> Vec<u8> {
        Vec::from(self.0.as_bytes())
    }
}
