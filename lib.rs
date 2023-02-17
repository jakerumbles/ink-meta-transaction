#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod flipper {
    // use ink::storage::Mapping;
    use ink_storage::Mapping;

    pub type Nonce = u128;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Flipper {
        /// Not a 256 bit integer as in the solidity version, but putting highest rust integer for now
        nonces: Mapping<AccountId, Nonce>,
    }

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
    }

    impl Flipper {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                nonces: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn get_nonce(&self, address: AccountId) -> Nonce {
            self.nonces.get(address).unwrap_or(0 as Nonce)
        }

        #[ink(message)]
        pub fn verfiy(&self) -> bool {
            true
        }

        #[ink(message)]
        pub fn execute(&mut self) -> (bool,) {
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
