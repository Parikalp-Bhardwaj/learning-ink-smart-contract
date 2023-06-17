#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#[ink::contract]
mod ink2 {
    use ink_prelude::string::String;
    #[ink(storage)]
    pub struct Ink2 {
        name: ink_prelude::string::String,
    }

    impl Ink2 {
        #[ink(constructor)]
        pub fn new(init_name: String) -> Self {
            Self { name: init_name }
        }

        #[ink(message)]
        pub fn get(&self) -> Option<String> {
            return Some(self.name.clone());
        }
        #[ink(message)]
        pub fn set(&mut self, init_name: String) {
            self.name = init_name;
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[ink::test]
        fn default_works() {
            let ink2 = Ink2::new(String::from("github"));
            assert_eq!(ink2.get(), Some("github".to_string()));
        }
        #[ink::test]
        fn default_works2() {
            let mut ink2 = Ink2::new(String::from("Git-Hub"));
            assert_eq!(ink2.get(), Some("Git-Hub".to_string()));
            ink2.set("Git".to_string());
            assert_eq!(ink2.get(), Some("Git".to_string()));
        }
    }
}
