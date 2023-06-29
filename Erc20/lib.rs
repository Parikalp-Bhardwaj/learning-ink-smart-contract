#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]

mod erc20 {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
    }

    pub type Result<T> = core::result::Result<T, Error>;
    impl Erc20 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });
            Self {
                total_supply,
                balances,
                allowances: Default::default(),
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_fn(&owner)
        }

        #[inline]
        fn balance_fn(&self, owner: &AccountId) -> Balance {
            self.balances.get(owner).unwrap_or(0)
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_fn(&owner, &spender)
        }

        #[inline]
        fn allowance_fn(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the caller's account balance.
        pub fn trasnfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.trasnfer_from_to(from, to, value)
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((&owner, &spender), &value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance_fn(&from, &caller);
            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }
            self.trasnfer_from_to(from, to, value)?;
            self.allowances
                .insert((&from, &caller), &(allowance - value));
            Ok(())
        }

        fn trasnfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let from_balance = self.balance_fn(&from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_fn(&to);
            self.balances.insert(to, &(to_balance + value));
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            Ok(())
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[ink::test]
        #[ink::test]
        fn total_supply_work() {
            let contract = Erc20::new(100);
            assert_eq!(contract.total_supply(), 100);
        }

        #[ink::test]
        fn balance_works() {
            let contract = Erc20::new(100);
            assert_eq!(contract.total_supply(), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 0);
        }
        #[ink::test]
        fn trasnfer_work() {
            let mut contract = Erc20::new(100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            let trasnfer = contract.trasnfer(AccountId::from([0x0; 32]), 50);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 50);
        }

        #[ink::test]
        fn transfer_from() {
            let mut contract = Erc20::new(100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            contract.approve(AccountId::from([0x1; 32]), 40);
            contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 30);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 30);
        }
    }
}
