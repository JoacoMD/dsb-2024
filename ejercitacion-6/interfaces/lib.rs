#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::primitives::AccountId;
use scale_info::prelude::string::String;

#[ink::trait_definition]
pub trait IRegistro {
    #[ink(message)]
    fn register(&mut self, name: String);
    #[ink(message)]
    fn get_user(&self, user: AccountId) -> String;
    #[ink(message)]
    fn user_exists(&self, user: AccountId) -> bool;
}
