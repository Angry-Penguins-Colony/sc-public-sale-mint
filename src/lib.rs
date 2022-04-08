#![no_std]

elrond_wasm::imports!();

pub const ERR_INIT_PRICE_PER_EGG_DIFF: &str = "Price per egg length different from max per wallet";
pub const ERR_INIT_PRICE_PER_EGG_ZERO: &str = "The price list is empty";
pub const ERR_INIT_REDUCED_PRICE_PER_EGG_DIFF: &str =
    "Reduced rice per egg length different from max per wallet";
pub const ERR_INIT_REDUCED_PRICE_PER_EGG_ZERO: &str = "The reduced price list is empty";

#[elrond_wasm::derive::contract]
pub trait PublicSaleMint {
    #[storage_mapper("max_per_wallet")]
    fn max_per_wallet(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("price_per_egg")]
    fn price_per_egg(&self) -> VecMapper<u64>;

    #[storage_mapper("reduced_price_per_egg")]
    fn reduced_price_per_egg(&self) -> VecMapper<u64>;

    #[storage_mapper("timestamp_public_sale")]
    fn timestamp_public_sale(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("timestamp_second_whitelist")]
    fn timestamp_second_whitelist(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("timestamp_first_whitelist")]
    fn timestamp_first_whitelist(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("token_identifier")]
    fn token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("token_nonce")]
    fn token_nonce(&self) -> SingleValueMapper<u64>;

    #[init]
    fn init(
        &self,
        max_per_wallet: u64,
        price_per_egg: ManagedVec<u64>,
        reduced_price_per_egg: ManagedVec<u64>,
        timestamp_public_sale: u64,
        second_whitelist_delta: u64,
        first_whitelist_delta: u64,
        token: TokenIdentifier,
        token_nonce: u64,
    ) {
        require!(price_per_egg.len() > 0, ERR_INIT_PRICE_PER_EGG_ZERO);

        require!(
            reduced_price_per_egg.len() > 0,
            ERR_INIT_REDUCED_PRICE_PER_EGG_ZERO
        );

        require!(
            price_per_egg.len() == max_per_wallet as usize,
            ERR_INIT_PRICE_PER_EGG_DIFF
        );

        require!(
            reduced_price_per_egg.len() == max_per_wallet as usize,
            ERR_INIT_REDUCED_PRICE_PER_EGG_DIFF
        );

        self.max_per_wallet().set(max_per_wallet);

        for price in price_per_egg.iter() {
            self.price_per_egg().push(&price);
        }

        for price in reduced_price_per_egg.iter() {
            self.reduced_price_per_egg().push(&price);
        }

        self.timestamp_public_sale().set(timestamp_public_sale);
        self.timestamp_second_whitelist()
            .set(timestamp_public_sale - second_whitelist_delta);
        self.timestamp_first_whitelist()
            .set(timestamp_public_sale - second_whitelist_delta - first_whitelist_delta);
        self.token_identifier().set(token);
        self.token_nonce().set(token_nonce);
    }
}
