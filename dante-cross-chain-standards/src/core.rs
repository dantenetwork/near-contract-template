use crate::types::Content;
use near_sdk::PromiseOrValue;

pub trait CrossChainCore {
    /// Register destination chain contract information.
    ///
    /// Arguments:
    /// * `chain_name`: the name of chain you want to send message.
    /// * `contract_address`: destination chain contract addrsss.
    /// * `action_name`: destination chain contract action name.
    fn register_dst_contract(
        &mut self,
        chain_name: String,
        contract_address: String,
        action_name: String,
    );

    /// Send a cross chain from current chain to `to_chain`.
    ///
    /// Arguments:
    /// * `to_chain`: the name of chain you want to send message.
    /// * `content`: the metadata of call destination chain contract.
    fn call_cross(&self, to_chain: String, content: Content);

    /// Send a cross chain from current chain to `to_chain`, and need receive a response.
    ///
    /// * `to_chain`: the name of chain you want to send message.
    /// * `content`: the metadata of call destination chain contract.
    fn call_cross_with_session(&self, to_chain: String, content: Content) -> PromiseOrValue<u64>;

    /// Send session response to `to_chain`.
    ///
    /// * `to_chain`: the name of chain you want to send message.
    /// * `content`: the metadata of call destination chain contract.
    /// * `id`: session id.
    fn send_response_message(&self, to_chain: String, content: Content, id: u64);
}
