/// The core methods for a basic Register cross information. Extension standards may be
/// added in addition to this macro.
#[macro_export]
macro_rules! impl_omni_chain_register {
    ($contract: ident, $cross: ident) => {
        use $crate::RegisterCore;

        #[near_bindgen]
        impl RegisterCore for $contract {
            fn register_dst_contract(
                &mut self,
                chain_name: String,
                action_name: String,
                contract_address: Vec<u8>,
                contract_action_name: Vec<u8>,
            ) {
                self.$cross.register_dst_contract(
                    chain_name,
                    action_name,
                    contract_address,
                    contract_action_name,
                );
            }

            fn register_permitted_contract(
                &mut self,
                chain_name: String,
                sender: Vec<u8>,
                action_name: String,
            ) {
                self.$cross
                    .register_permitted_contract(chain_name, sender, action_name);
            }
        }
    };
}
