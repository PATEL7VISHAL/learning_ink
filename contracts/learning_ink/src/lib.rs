#![cfg_attr(not(feature = "std"), no_std)]

///Here we use the native tranfer function to tranfer native tokens to the contract.
#[ink::contract]
mod learning_ink {
    #[ink(storage)]
    pub struct LearningInk {
        owner: AccountId,
    }

    impl LearningInk {
        #[ink(constructor)]
        pub fn new() -> Self {
            LearningInk {
                owner: Self::env().caller(),
            }
        }

        ///Value is sended from the fontend (the by the payable macro allow us to send token from
        ///font-end)
        #[ink(message, payable)]
        pub fn transfer(&self, value: Balance) {
            let transferred_value = self.env().transferred_value();
            let caller = self.env().caller();

            ink::env::debug_println!("Requested Amount: {}", value);
            ink::env::debug_println!("Payer Sended amount: {}", transferred_value);
            ink::env::debug_println!("Current Contract Balance: {}", self.env().balance());

            if transferred_value < value {
                //NOTE: here value can be anythink may comes from state
                panic!("You haven't sended enough token for tranfer");
            } else if transferred_value > value {
                ink::env::debug_println!("You have sended more token then it required");
                let extra = transferred_value - value;
                if self.env().transfer(caller, extra).is_err() {
                    panic!("Unable to return back amount to you")
                }
            }
            ink::env::debug_println!("Tx sussfull")
        }

        ///Get the token amount of contract
        #[ink(message)]
        pub fn get_balance(&self) -> Balance {
            let balance = self.env().balance();
            ink::env::debug_println!("Contract Balance : {}", balance);

            balance
        }

        #[ink(message)]
        pub fn tranfer_to_owner(&self) {
            ink::env::debug_println!("Contract Balance : {}", self.env().balance());
            ink::env::debug_println!("Left Gas : ~{}", self.env().gas_left());

            // assert!(value <= self.env().balance(), "insufficient funds!");

            if self
                .env()
                // .transfer(self.owner, self.env().balance())
                .transfer(self.owner, 200)
                .is_err()
            {
                panic!("Failed to transfer token from contract to owner");
            }
        }
    }
}
