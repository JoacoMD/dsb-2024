#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod wallet {

    #[ink(storage)]
    pub struct Wallet {
        owner: AccountId,
    }

    impl Wallet {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { owner: Self::env().caller() }
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) {
            let caller = self.env().caller();
            let transferred_value = self.env().transferred_value();
            if let Err(e) = self.env().transfer(caller, transferred_value) {
                panic!("Transfer failed: {:?}", e);
            }
        }

        #[ink(message)]
        pub fn retire(&mut self, destination: AccountId, amount: Balance) {
            self.only_owner();
            if amount > self.env().balance() {
                panic!("Not enough balance");
            }
            if let Err(e) = self.env().transfer(destination, self.env().balance()) {
                panic!("Transfer failed: {:?}", e);
            }
        }

        #[ink(message)]
        pub fn balance(&self) -> u128 {
            self.only_owner();
            self.env().balance()
        }

        fn only_owner(&self) {
            if self.env().caller() != self.owner {
                panic!("Only owner can call this function");
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

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let wallet = Wallet::new();
            assert_eq!(wallet.balance(), 1000000);
        }

        #[ink::test]
        fn retire_works() {
            let mut wallet = Wallet::new();
            assert_eq!(wallet.balance(), 1000000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            wallet.retire(accounts.bob, 1000000);
            assert_eq!(wallet.balance(), 0);
        }

        #[ink::test]
        #[should_panic(expected = "Only owner can call this function")]
        fn balance_fails() {
            let wallet = Wallet::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert_eq!(wallet.balance(), 1000000);
        }

        #[ink::test]
        fn deposit_works() {
            let mut wallet = Wallet::new();
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000000);
            wallet.deposit();
            assert_eq!(wallet.balance(), 2000000);
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
            let mut constructor = WalletRef::default();

            // When
            let contract = client
                .instantiate("wallet", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<Wallet>();

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = WalletRef::new(false);
            let contract = client
                .instantiate("wallet", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Wallet>();

            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = call_builder.flip();
            let _flip_result = client
                .call(&ink_e2e::bob(), &flip)
                .submit()
                .await
                .expect("flip failed");

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
