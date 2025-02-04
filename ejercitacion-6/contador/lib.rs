#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod contador {

    use ink::storage::Mapping;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Contador {
        /// Stores a single `bool` value on the storage.
        value: i128,
        owner: AccountId,
        whitelist: ink::storage::Mapping<AccountId, bool>
    }

    impl Contador {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut whitelist = Mapping::default();
            whitelist.insert(Self::env().caller(), &true);
            Self { 
                value: 0,
                owner: Self::env().caller(), 
                whitelist 
            }
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn increment(&mut self) {
            self.in_whitelist();
            if self.value == i128::MAX {
                panic!("Value is already at maximum");
            } else {
                self.value += 1;
            }
        }

        #[ink(message)]
        pub fn decrement(&mut self) {
            self.in_whitelist();
            if self.value > i128::MIN {
                self.value -= 1;
            } else {
                panic!("Value is already at minimum");
            }
        }

        #[ink(message)]
        pub fn add_to_whitelist(&mut self, account: AccountId) {
            self.only_owner();
            self.whitelist.insert(account, &true);
        }

        #[ink(message)]
        pub fn remove_from_whitelist(&mut self, account: AccountId) {
            self.only_owner();
            self.whitelist.remove(account);
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> i128 {
            self.value
        }

        fn only_owner(&self) {
            if self.env().caller() != self.owner {
                panic!("Only owner can call this function");
            }
        }

        fn in_whitelist(&self) {
            if !self.whitelist.contains(self.env().caller()) {
                panic!("Caller is not in the whitelist");
            }
        }
    }

    
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let contador = Contador::new();
            assert_eq!(contador.get(), 0);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn increment_works() {
            let mut contador = Contador::new();
            assert_eq!(contador.get(), 0);
            contador.increment();
            assert_eq!(contador.get(), 1);
        }

        #[ink::test]
        fn decrement_works() {
            let mut contador = Contador::new();
            assert_eq!(contador.get(), 0);
            contador.increment();
            assert_eq!(contador.get(), 1);
            contador.decrement();
            assert_eq!(contador.get(), 0);
        }

        #[ink::test]
        #[should_panic]
        fn only_owner_works() {
            let mut contador = Contador::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            contador.add_to_whitelist(accounts.charlie);
        }

        #[ink::test]
        #[should_panic]
        fn in_whitelist_works() {
            let mut contador = Contador::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            contador.increment();
        }

        #[ink::test]
        fn add_to_whitelist_works() {
            let mut contador = Contador::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            contador.add_to_whitelist(accounts.bob);
            
            assert_eq!(contador.whitelist.contains(accounts.bob), true);
        }

        #[ink::test]
        fn remove_to_whitelist_works() {
            let mut contador = Contador::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            contador.add_to_whitelist(accounts.bob);
            
            assert_eq!(contador.whitelist.contains(accounts.bob), true);

            contador.remove_from_whitelist(accounts.bob);

            assert_eq!(contador.whitelist.contains(accounts.bob), false);
        }
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::ContractsBackend;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = ContadorRef::new();

            // When
            let contract = client
                .instantiate("contador", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<Contador>();

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), 0));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = ContadorRef::new();
            let contract = client
                .instantiate("contador", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Contador>();

            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), 0));

            // When
            let increment = call_builder.increment();
            let _increment_result = client
                .call(&ink_e2e::bob(), &increment)
                .submit()
                .await
                .expect("increment failed");

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), 1));

            Ok(())
        }
    }
}
