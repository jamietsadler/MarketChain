#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod Intraday {
    #[ink(storage)]
    pub struct IntradayContract {
        // Enhanced storage struct with new fields
        trades: ink_storage::collections::HashMap<TradeId, TradeDetails>,
    }


    #[derive(scale::Encode, scale::Decode, Debug, Clone, PartialEq)]
    pub struct TradeDetails {
        pub remote_trade_id: String,
        pub side: String,
        pub product: String,
        pub delivery_start: Timestamp,
        pub delivery_end: Timestamp,
        pub execution_time: Timestamp,
        pub delivery_area: String,
        pub trade_phase: String,
        pub user_defined_block: bool,
        pub self_trade: bool,
        pub price: Balance,
        pub currency: String,
        pub volume: u64,
        pub volume_unit: String,
        pub order_id: String,
    }

    pub type TradeId = u32;
    pub type Timestamp = u64;

    impl IntradayContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                trades: Default::default(),
            }
        }

        #[ink(message)]
        pub fn create_trade(&mut self, trade_id: TradeId, details: TradeDetails) {
            let caller = self.env().caller();
            // Additional logic for validation or access control
            assert!(self.trades.insert(trade_id, details).is_none(), "Trade already exists");
        }

        #[ink(message)]
        pub fn get_trade_details(&self, trade_id: TradeId) -> Option<TradeDetails> {
            self.trades.get(&trade_id).cloned()
        }
    }
}
