pub mod core_impl;
pub mod macros;
mod types;

pub use self::core_impl::OmniChain;
pub use self::types::*;

pub trait RegisterCore {
    fn register_permitted_contract(
        &mut self,
        chain_name: String,
        sender: Vec<u8>,
        action_name: String,
    );

    fn register_dst_contract(
        &mut self,
        action_name: String,
        chain_name: String,
        contract_address: Vec<u8>,
        contract_action_name: Vec<u8>,
    );
}
