#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod inkmetatransaction {
    use ink::env::{
        call::{build_call, ExecutionInput},
        CallFlags,
    };
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use scale::{Encode, Output};

    pub type Nonce = u128;

    #[derive(scale::Decode, scale::Encode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink::storage::traits::StorageLayout
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
        /// Submitted nonce. Must match what is expected on-chain or transaction is invalid.
        pub nonce: Nonce,
        pub expiration_time_seconds: Timestamp,
    }

    /// A wrapper that allows us to encode a blob of bytes.
    ///
    /// We use this to pass the set of untyped (bytes) parameters to the `CallBuilder`.
    struct CallInput<'a>(&'a [u8]);

    impl<'a> scale::Encode for CallInput<'a> {
        fn encode_to<T: Output + ?Sized>(&self, dest: &mut T) {
            dest.write(self.0);
        }
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Copy, Clone, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Signature does not match transaction
        BadSignature,
        /// Returned if the call failed.
        TransactionFailed,
        ValueTransferMismatch,
        TransactionExpired,
        // Submitted nonce does match expected
        IncorrectNonce,
        // Signature does not match transaction
        IncorrectSignature,
    }

    #[ink(storage)]
    pub struct InkMetaTransaction {
        /// Not a 256 bit integer as in the solidity version, but putting highest rust integer for now
        nonces: Mapping<AccountId, Nonce>,
    }

    impl InkMetaTransaction {
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
        pub fn verfiy(&self, req: Transaction, signature: [u8; 65]) -> Result<bool, Error> {
            ink::env::debug_println!("req.callee: {:?}", req.callee);

            let encoded_msg: Vec<u8> = req.encode();
            ink::env::debug_println!("Encoded message Vec<u8>: {:?}", encoded_msg);
            // let message_hash = Keccak256::digest(encoded_msg).to_vec();
            let message_hash = Self::keccak256_hash(encoded_msg);
            ink::env::debug_println!("32 byte message hash: {:?}", message_hash);

            match self.env().ecdsa_recover(&signature, &message_hash) {
                Ok(pub_key) => {
                    // Match recovered pub_key with caller
                    let caller = self.env().caller();
                    let acc_id = Self::to_default_account_id(pub_key);

                    let expected_nonce = self.get_nonce(caller);

                    // Is the message signed by the same account that sent it?
                    // And does the transacation have the expected nonce?
                    // if expected_nonce == req.nonce && acc_id == caller_bytes {
                    if expected_nonce != req.nonce {
                        return Err(Error::IncorrectNonce);
                    }
                    if acc_id != caller {
                        return Err(Error::IncorrectSignature);
                    } else {
                        return Ok(true);
                    }
                }
                Err(_) => return Err(Error::IncorrectSignature),
            }
        }

        #[ink(message, payable)]
        pub fn execute(&mut self, req: Transaction, signature: [u8; 65]) -> Result<(), Error> {
            // Signature must be correct
            if let Err(_) = self.verfiy(req.clone(), signature.clone()) {
                return Err(Error::BadSignature);
            }

            // Assert that the correct amount of tokens were sent to this contract instance with this fn call
            if self.env().transferred_value() != req.transferred_value {
                return Err(Error::ValueTransferMismatch);
            }

            // Assert that the transaction hasn't already expired
            if self.env().block_timestamp() >= req.expiration_time_seconds {
                return Err(Error::TransactionExpired);
            }

            let caller = self.env().caller();
            let updated_nonce = self.get_nonce(caller) + 1;

            // Signature is valid, so increase nonce and then execute transaction
            self.nonces.insert(caller, &updated_nonce);

            let result = build_call::<<Self as ::ink::env::ContractEnv>::Env>()
                .call(req.callee)
                .gas_limit(req.gas_limit)
                .transferred_value(req.transferred_value)
                .call_flags(CallFlags::default().set_allow_reentry(req.allow_reentry))
                .exec_input(
                    ExecutionInput::new(req.selector.into()).push_arg(CallInput(&req.input)),
                )
                .returns::<()>()
                .try_invoke();

            let result = match result {
                Ok(Ok(_)) => Ok(()),
                _ => Err(Error::TransactionFailed),
            };

            result
        }

        fn to_default_account_id(compressed_pub_key: [u8; 33]) -> ink::primitives::AccountId {
            use ink::env::hash;

            let mut output = <hash::Blake2x256 as hash::HashOutput>::Type::default();
            ink::env::hash_bytes::<hash::Blake2x256>(&compressed_pub_key[..], &mut output);

            output.into()
        }

        fn keccak256_hash(bytes: Vec<u8>) -> [u8; 32] {
            use ink::env::hash;

            let mut output = <hash::Keccak256 as hash::HashOutput>::Type::default();
            ink::env::hash_bytes::<hash::Keccak256>(&bytes[..], &mut output);

            output
        }
    }

    // #[cfg(test)]
    // mod tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;

    //     /// We test if the default constructor does its job.
    //     #[ink::test]
    //     fn default_works() {
    //         let flipper = Flipper::default();
    //         assert_eq!(flipper.get(), false);
    //     }

    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut flipper = Flipper::new(false);
    //         assert_eq!(flipper.get(), false);
    //         flipper.flip();
    //         assert_eq!(flipper.get(), true);
    //     }
    // }

    // /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    // ///
    // /// When running these you need to make sure that you:
    // /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    // /// - Are running a Substrate node which contains `pallet-contracts` in the background
    // #[cfg(all(test, feature = "e2e-tests"))]
    // mod e2e_tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;

    //     /// A helper function used for calling contract messages.
    //     use ink_e2e::build_message;

    //     /// The End-to-End test `Result` type.
    //     type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    //     /// We test that we can upload and instantiate the contract using its default constructor.
    //     #[ink_e2e::test]
    //     async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    //         // Given
    //         let constructor = FlipperRef::default();

    //         // When
    //         let contract_account_id = client
    //             .instantiate("flipper", &ink_e2e::alice(), constructor, 0, None)
    //             .await
    //             .expect("instantiate failed")
    //             .account_id;

    //         // Then
    //         let get = build_message::<FlipperRef>(contract_account_id.clone())
    //             .call(|flipper| flipper.get());
    //         let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
    //         assert!(matches!(get_result.return_value(), false));

    //         Ok(())
    //     }

    //     /// We test that we can read and write a value from the on-chain contract contract.
    //     #[ink_e2e::test]
    //     async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    //         // Given
    //         let constructor = FlipperRef::new(false);
    //         let contract_account_id = client
    //             .instantiate("flipper", &ink_e2e::bob(), constructor, 0, None)
    //             .await
    //             .expect("instantiate failed")
    //             .account_id;

    //         let get = build_message::<FlipperRef>(contract_account_id.clone())
    //             .call(|flipper| flipper.get());
    //         let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
    //         assert!(matches!(get_result.return_value(), false));

    //         // When
    //         let flip = build_message::<FlipperRef>(contract_account_id.clone())
    //             .call(|flipper| flipper.flip());
    //         let _flip_result = client
    //             .call(&ink_e2e::bob(), flip, 0, None)
    //             .await
    //             .expect("flip failed");

    //         // Then
    //         let get = build_message::<FlipperRef>(contract_account_id.clone())
    //             .call(|flipper| flipper.get());
    //         let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
    //         assert!(matches!(get_result.return_value(), true));

    //         Ok(())
    //     }
    // }
}
