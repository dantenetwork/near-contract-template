/// The core methods for a basic fungible token. Extension standards may be
/// added in addition to this macro.
#[macro_export]
macro_rules! impl_cross_chain_core {
    ($contract: ident, $cross: ident) => {
        use $crate::core::CrossChainCore;
        // use $crate::fungible_token::resolver::FungibleTokenResolver;

        #[near_bindgen]
        impl CrossChainCore for $contract {
            fn register_dst_contract(
                &mut self,
                chain_name: String,
                contract_address: String,
                action_name: String,
            ) {
                self.$cross
                    .register_dst_contract(chain_name, contract_address, action_name)
            }

            fn call_cross(&self, to_chain: String, content: Content) {
                self.$cross.call_cross(to_chain, content)
            }

            fn call_cross_with_session(
                &self,
                to_chain: String,
                content: Content,
            ) -> PromiseOrValue<u64> {
                self.$token.call_cross_with_session(to_chain, content)
            }

            fn send_response_message(&self, to_chain: String, content: Content, id: u64) {
                self.$token.send_response_message(to_chain, content, id)
            }
        }
    };
}
