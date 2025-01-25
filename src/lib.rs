#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod contracts; // bring in contracts

#[ink::contract]
mod market_chain {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use scale::{Decode, Encode};
    
    #[ink(storage)]
    pub struct MarketChain {
        orders: Vec<(u32, Order)>,
        orders_mapping: Mapping<u32, Order>,

    }

}
