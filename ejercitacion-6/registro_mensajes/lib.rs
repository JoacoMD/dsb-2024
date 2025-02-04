#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod registro_mensajes {

    use ink::storage::Mapping;
    use interfaces::IRegistro;
    use scale_info::prelude::string::String;

    #[ink(storage)]
    pub struct RegistroMensajes {
        /// Stores a single `bool` value on the storage.
        users: Mapping<AccountId, String>,
    }

    impl RegistroMensajes {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { users: Default::default() }
        }
    }

    impl IRegistro for RegistroMensajes {
        
        #[ink(message)]
        fn register(&mut self, name: String) {
            let caller = self.env().caller();
            if self.users.contains(&caller) {
                panic!("User already exists");
            }
            self.users.insert(caller, &name);
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        fn get_user(&self, user: AccountId) -> String {
            let user = self.users.get(&user).unwrap_or_else(|| panic!("User not found"));
            user
        }

        #[ink(message)]
        fn user_exists(&self, user: AccountId) -> bool {
            self.users.contains(&user)
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[ink::test]
        fn it_works() {
            let mut registro_mensajes = RegistroMensajes::new();
            registro_mensajes.register("Alice".to_string());
            let caller = ink::env::test::callee::<ink::env::DefaultEnvironment>();
            assert_eq!(registro_mensajes.get_user(caller), "Alice".to_string());
        }

        #[ink::test]
        fn user_exists() {
            let mut registro_mensajes = RegistroMensajes::new();
            let caller = ink::env::test::callee::<ink::env::DefaultEnvironment>();
            assert_eq!(registro_mensajes.user_exists(caller), false);
            registro_mensajes.register("Alice".to_string());
            assert_eq!(registro_mensajes.user_exists(caller), true);
        }

        #[ink::test]
        #[should_panic(expected = "User already exists")]
        fn user_already_exists() {
            let mut registro_mensajes = RegistroMensajes::new();
            registro_mensajes.register("Alice".to_string());
            registro_mensajes.register("Alice".to_string());
        }

        #[ink::test]
        #[should_panic(expected = "User not found")]
        fn user_not_found() {
            let registro_mensajes = RegistroMensajes::new();
            let caller = ink::env::test::callee::<ink::env::DefaultEnvironment>();
            registro_mensajes.get_user(caller);
        }
    }


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
    //     use ink_e2e::ContractsBackend;

    //     /// The End-to-End test `Result` type.
    //     type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    //     /// We test that we can upload and instantiate the contract using its default constructor.
    //     #[ink_e2e::test]
    //     async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    //         // Given
    //         let mut constructor = RegistroMensajesRef::default();

    //         // When
    //         let contract = client
    //             .instantiate("registro_mensajes", &ink_e2e::alice(), &mut constructor)
    //             .submit()
    //             .await
    //             .expect("instantiate failed");
    //         let call_builder = contract.call_builder::<RegistroMensajes>();

    //         // Then
    //         let get = call_builder.get();
    //         let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
    //         assert!(matches!(get_result.return_value(), false));

    //         Ok(())
    //     }

    //     /// We test that we can read and write a value from the on-chain contract.
    //     #[ink_e2e::test]
    //     async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    //         // Given
    //         let mut constructor = RegistroMensajesRef::new(false);
    //         let contract = client
    //             .instantiate("registro_mensajes", &ink_e2e::bob(), &mut constructor)
    //             .submit()
    //             .await
    //             .expect("instantiate failed");
    //         let mut call_builder = contract.call_builder::<RegistroMensajes>();

    //         let get = call_builder.get();
    //         let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
    //         assert!(matches!(get_result.return_value(), false));

    //         // When
    //         let flip = call_builder.flip();
    //         let _flip_result = client
    //             .call(&ink_e2e::bob(), &flip)
    //             .submit()
    //             .await
    //             .expect("flip failed");

    //         // Then
    //         let get = call_builder.get();
    //         let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
    //         assert!(matches!(get_result.return_value(), true));

    //         Ok(())
    //     }
    // }
}
