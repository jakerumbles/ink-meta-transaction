#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod ink_meta_transaction {
    // use ink_env::AccountId;
    // use ink::storage::Mapping;
    use ink_storage::Mapping;
    use sha3::{Digest, Keccak256};

    pub type Nonce = u128;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct InkMetaTransaction {
        /// Not a 256 bit integer as in the solidity version, but putting highest rust integer for now
        nonces: Mapping<AccountId, Nonce>,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink_storage::traits::StorageLayout
        )
    )]
    pub struct Transaction {
        /// The `AccountId` of the contract that is called in this transaction.
        pub callee: AccountId,
        /// The selector bytes that identifies the function of the callee that should be called.
        pub selector: [u8; 4],
        /// The SCALE encoded parameters that are passed to the called function.
        pub input: Vec<u8>,
        /// The amount of chain balance that is transferred to the callee.
        pub transferred_value: Balance,
        /// Gas limit for the execution of the call.
        pub gas_limit: u64,
        /// If set to true the transaction will be allowed to re-enter the multisig contract.
        /// Re-entrancy can lead to vulnerabilities. Use at your own risk.
        pub allow_reentry: bool,
        /// Submitted account nonce
        /// Must match what is expected on-chain
        pub nonce: Nonce,
    }

    // let typehash: [u8; 32] = [0u8];

    // bytes32 private constant _TYPEHASH =
    //     keccak256("ForwardRequest(address from,address to,uint256 value,uint256 gas,uint256 nonce,bytes data)");

    fn typehash() -> [u8; 32] {
        // let mut hasher = sha3::Keccak256::default();
        let mut hasher = Keccak256::new();

        // Input data to the hasher
        hasher.update("Transaction{callee:AccountId,selector:[u8;4],input:Vec<u8>,transferred_value:Balance,gas_limit:u64,allow_reentry:bool,}");

        hasher.finalize().into()
    }

    // pub struct Transaction {
    //     pub from: AccountId,
    //     pub to: AccountId,
    //     pub value: Balance,
    //     pub gas: Balance,
    //     pub nonce: Nonce,
    //     pub data: [u8; 32],
    // }

    impl InkMetaTransaction {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                nonces: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn get_nonce(&self, address: AccountId) -> Nonce {
            self.nonces.get(address).unwrap_or(0 as Nonce)
        }

        #[ink(message)]
        pub fn verfiy(&self, req: Transaction, signature: [u8; 32]) -> bool {
            match self.env().ecdsa_recover(signature, message_hash) {
                Ok(pub_key) => {
                    // Match pub_key with something
                    true
                }
                Err(_) => return false,
            }
            // true
        }

        #[ink(message)]
        pub fn execute(&mut self) -> bool {
            true
        }

        // #[ink(message)]
        // pub fn flip(&mut self) {
        //     self.value = !self.value;
        // }

        // /// Simply returns the current value of our `bool`.
        // #[ink(message)]
        // pub fn get(&self) -> bool {
        //     self.value
        // }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let meta = InkMetaTransaction::default();
            assert_eq!(meta.get_nonce(AccountId::from([0; 32])), 0);
            assert_eq!(meta.get_nonce(AccountId::from([9; 32])), 0);
        }

        // #[ink::test]
        // fn default_works() {
        //     let flipper = Flipper::default();
        //     assert_eq!(flipper.get(), false);
        // }

        // #[ink::test]
        // fn it_works() {
        //     let mut flipper = Flipper::new(false);
        //     assert_eq!(flipper.get(), false);
        //     flipper.flip();
        //     assert_eq!(flipper.get(), true);
        // }
    }
}
