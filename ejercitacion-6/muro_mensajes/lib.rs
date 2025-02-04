#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod muro_mensajes {

    use ink::storage::StorageVec;
    use interfaces::IRegistro;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct MuroMensajes {
        /// Stores a single `bool` value on the storage.
        muro: StorageVec<Message>,
        registro_contract: ink::contract_ref!(IRegistro),
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout, Debug))]
    pub struct Message {
        author: AccountId,
        content: String,
    }

    impl MuroMensajes {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(registro: AccountId) -> Self {
            Self {
                muro: Default::default(),
                registro_contract: registro.into(),
            }
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn publicar(&mut self, msj: String) {
            let author = self.env().caller();
            if !self.registro_contract.user_exists(author) {
                panic!("User not registered");
            }
            let mesg = Message {
                author,
                content: msj,
            };
            self.muro.push(&mesg);
        }

        #[ink(message)]
        pub fn count(&self) -> u32 {
            self.muro.len()
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
            let mock_registro = AccountId::from([0x1; 32]);
            let muro_mensajes = MuroMensajes::new(mock_registro);
            assert_eq!(muro_mensajes.count(), 0);
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
        use mock_registro::MockRegistroRef;

        /// A helper function used for calling contract messages.
        use ink_e2e::ContractsBackend;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            
            let constructor_mock_registro = MockRegistroRef::default();
            let account = ink_e2e::alice();
            let value = 0;
            let name = "mock_registro";
            let storage_limit_deposit = None;
            let contract_address_mock_registro = client.instantiate(
                name,
                &account,
                constructor_mock_registro,
                value,
                storage_limit_deposit,
            )
                .await
                .expect("instantiate failed")
                .account_id;
            
            let constructor = MuroMensajesRef::new(contract_address_mock_registro);
            let account = ink_e2e::alice();
            let name = "muro_mensajes";
            let contract_address_muro_mensajes = client.instantiate(
                name,
                &account,
                constructor,
                value,
                storage_limit_deposit,
            )
                .await
                .expect("instantiate failed")
                .account_id;

            let message_count = build_message::<MuroMensajesRef>(contract_address_muro_mensajes.clone())
                .call(|muro| muro.count());
            let message_count_result = client.call_dry_run(&account, &message_count,value, storage_limit_deposit).await;

            assert!(matches!(message_count_result.return_value(), 0));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor_mock_registro = MockRegistroRef::default();
            let account = ink_e2e::alice();
            let value = 0;
            let name = "mock_registro";
            let storage_limit_deposit = None;
            let contract_address_mock_registro = client.instantiate(
                name,
                &account,
                constructor_mock_registro,
                value,
                storage_limit_deposit,
            )
                .await
                .expect("instantiate failed")
                .account_id;
            
            let constructor = MuroMensajesRef::new(contract_address_mock_registro);
            let account = ink_e2e::alice();
            let name = "muro_mensajes";
            let contract_address_muro_mensajes = client.instantiate(
                name,
                &account,
                constructor,
                value,
                storage_limit_deposit,
            )
                .await
                .expect("instantiate failed")
                .account_id;

            let message_publicar = build_message::<MuroMensajesRef>(contract_address_muro_mensajes.clone())
                .call(|muro| muro.publicar("Hola".to_string()));
            let message_publicar_result = client.call_dry_run(&account, &message_publicar, value, storage_limit_deposit).await;


            let message_count = build_message::<MuroMensajesRef>(contract_address_muro_mensajes.clone())
                .call(|muro| muro.count());
            let message_count_result = client.call_dry_run(&account, &message_count,value, storage_limit_deposit).await;

            assert!(matches!(message_count_result.return_value(), 1));

            Ok(())
        }
    }
}
