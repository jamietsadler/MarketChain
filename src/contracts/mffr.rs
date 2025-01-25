#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod MFFR {
    #[ink(storage)]
    pub struct MFFRContract {
        trades: ink_storage::collections::HashMap<TradeId, TradeDetails>,
    }
    
    pub type TradeId = u32;
    pub type Price = u128;

    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct TradeDetails {
        pub tender_round: String,
        pub status: String,
        pub rejection_code: String,
        pub company_name: String,
        pub bmuid: String,
        pub generation_type: String,
        pub start_date: u64,
        pub end_date: u64,
        pub p02level: u32,
        pub p05level: u32,
        pub p08level: u32,
        pub s02level: u32,
        pub s05level: u32,
        pub h02level: u32,
        pub h05level: u32,
        pub dynamic_flag: bool,
        pub bid_price: Price,
        pub day: String,
        pub efa: String,
        pub price_per_mw: Price,
    }

    impl MFFRContract {
        /// Constructor to initialize the contract
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                trades: ink_storage::collections::HashMap::new(),
            }
        }

        /// Add a new trade
        #[ink(message)]
        pub fn add_trade(&mut self, trade_id: TradeId, details: TradeDetails) -> bool {
            if self.trades.contains_key(&trade_id) {
                return false; // Trade ID already exists
            }
            self.trades.insert(trade_id, details);
            true
        }

        /// Get details of a trade by ID
        #[ink(message)]
        pub fn get_trade(&self, trade_id: TradeId) -> Option<TradeDetails> {
            self.trades.get(&trade_id).cloned()
        }
                                
        /// Delete a trade by ID
        #[ink(message)]
        pub fn delete_trade(&mut self, trade_id: TradeId) -> bool {
            self.trades.remove(&trade_id).is_some()
        }
    }
}
