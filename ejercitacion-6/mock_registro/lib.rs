#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod mock_registro {

    use ink::storage::Mapping;
    use interfaces::IRegistro;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct MockRegistro {
        /// Stores a single `bool` value on the storage.
        users: Mapping<AccountId, String>,
    }

    impl MockRegistro {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { users: Default::default() }
        }
    }

    impl IRegistro for MockRegistro {
        
        #[ink(message)]
        fn register(&mut self, name: String) {
            
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        fn get_user(&self, user: AccountId) -> String {
            "Alice".to_string()
        }

        #[ink(message)]
        fn user_exists(&self, user: AccountId) -> bool {
            true
        }
    }

}
