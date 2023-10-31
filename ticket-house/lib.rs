#![cfg_attr(not(feature = "std"), no_std, no_main)]

extern crate alloc;

pub use ticket_office::__ink_EventBase as Event;
pub use ticket_office::{Extracted, Recharged, TicketOffice};

#[pink::contract(env = PinkEnvironment)]
mod ticket_office {
    use pink::PinkEnvironment;
    use alloc::string::String;

    type Result<T> = core::result::Result<T, Error>;

    #[ink(storage)]
    pub struct TicketOffice {
        /// Owner of the contract
        owner: AccountId,
    }

    #[ink(event)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Extracted {
        pub dest: AccountId,
        pub amount: Balance,
    }

    #[ink(event)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Recharged {
        pub account: String,
        pub amount: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        BadOrigin,
        TransferFailed,
    }

    impl TicketOffice {
        fn ensure_owner(&self) -> Result<()> {
            if self.env().caller() != self.owner {
                return Err(Error::BadOrigin);
            }
            Ok(())
        }

        #[ink(constructor)]
        #[allow(clippy::should_implement_trait)]
        pub fn default() -> Self {
            Self {
                owner: Self::env().caller(),
            }
        }

        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message)]
        pub fn version(&self) -> this_crate::VersionTuple {
            this_crate::version_tuple!()
        }

        #[ink(message)]
        pub fn extract(&mut self, dest: AccountId, amount: Balance) -> Result<()> {
            self.ensure_owner()?;
            self.env()
                .transfer(dest, amount)
                .or(Err(Error::TransferFailed))?;
            pink::info!("Extracted {amount} to {dest:?}");
            self.env().emit_event(Extracted { dest, amount });
            Ok(())
        }

        #[ink(message, payable)]
        pub fn recharge(&mut self, account: String) -> Result<()> {
            let amount = self.env().transferred_value();
            if amount == 0 {
                return Ok(());
            }
            pink::info!("Recharged {account:?} with {amount}.");
            self.env().emit_event(Recharged { account, amount });
            Ok(())
        }

        /// For self upgrade.
        #[ink(message)]
        pub fn set_code(&mut self, code_hash: pink::Hash) -> Result<()> {
            self.ensure_owner()?;
            ink::env::set_code_hash(&code_hash).expect("Failed to set code hash");
            pink::info!("Switched code hash to {:?}.", code_hash);
            Ok(())
        }
    }
}
