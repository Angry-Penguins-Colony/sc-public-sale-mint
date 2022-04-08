#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::derive::contract]
pub trait PublicSaleMint {
    #[init]
    fn init(
        &self,
        max_per_wallet: u64,
        price_per_egg: ManagedVec<u64>,
        reduced_price_per_egg: ManagedVec<u64>,
        timestamp_public_sale: u64,
        second_whitelist_delta: u64,
        first_whitelist_delta: u64,
    ) {
        require!(
            price_per_egg.len() == reduced_price_per_egg.len(),
            "Price per egg length different from max per wallet"
        );

        panic!("init not implemented");
    }
}
