#![no_std]

elrond_wasm::imports!();

pub const ERR_INIT_PRICE_PER_EGG_DIFF: &str = "Price per egg length different from max per wallet";
pub const ERR_INIT_PRICE_PER_EGG_ZERO: &str = "The price list is empty";
pub const ERR_INIT_REDUCED_PRICE_PER_EGG_DIFF: &str =
    "Reduced rice per egg length different from max per wallet";
pub const ERR_INIT_REDUCED_PRICE_PER_EGG_ZERO: &str = "The reduced price list is empty";
pub const ERR_INIT_SECOND_WL_LESSER_THEN_FIRST: &str =
    "The second whitelist must be lesser than the first";

pub mod whitelist;

pub const ERR_NOT_OWNER: &str = "Endpoint can only be called by owner";
pub const ERR_FILL_BAD_NONCE: &str =
    "The nonce you are trying to fill the SC with is not the one expected";
pub const ERR_FILL_BAD_IDENTIFIER: &str =
    "The identifier of the token you are trying to fill is not the one expected";

#[elrond_wasm::derive::contract]
pub trait PublicSaleMint: whitelist::WhitelistModule {
    #[storage_mapper("max_per_wallet")]
    fn max_per_wallet(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("price_per_egg")]
    fn price_per_egg(&self) -> VecMapper<u64>;

    #[storage_mapper("reduced_price_per_egg")]
    fn reduced_price_per_egg(&self) -> VecMapper<u64>;

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

        require!(
            second_whitelist_delta < first_whitelist_delta,
            ERR_INIT_SECOND_WL_LESSER_THEN_FIRST
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
            .set(timestamp_public_sale - first_whitelist_delta);
        self.token_identifier().set(token);
        self.token_nonce().set(token_nonce);
    }

    #[endpoint]
    #[payable("*")]
    #[only_owner]
    fn fill_egg(
        &self,
        #[payment] _payment: BigUint,
        #[payment_token] token: TokenIdentifier,
        #[payment_nonce] nonce: u64,
    ) {
        self.blockchain().check_caller_is_owner();

        require!(
            self.token_identifier().get() == token,
            ERR_FILL_BAD_IDENTIFIER
        );

        require!(self.token_nonce().get() == nonce, ERR_FILL_BAD_NONCE);
    }

    #[endpoint]
    #[payable("*")]
    fn buy(
        &self,
        #[payment] _payment: BigUint,
        #[payment_token] _token: TokenIdentifier,
        #[payment_nonce] _nonce: u64,
    ) {
        panic!("Not implemented");
    }
}
