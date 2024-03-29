#![no_std]

elrond_wasm::imports!();

pub const ERR_INIT_PRICE_PER_EGG_DIFF: &str = "Price per egg length different from max per wallet";
pub const ERR_INIT_PRICE_PER_EGG_ZERO: &str = "The price list is empty";
pub const ERR_INIT_REDUCED_PRICE_PER_EGG_DIFF: &str =
    "Reduced rice per egg length different from max per wallet";
pub const ERR_INIT_REDUCED_PRICE_PER_EGG_ZERO: &str = "The reduced price list is empty";
pub const ERR_INIT_SECOND_WL_LESSER_THEN_FIRST: &str =
    "The second whitelist must be lesser or equal than the first";

pub mod whitelist;

pub const ERR_NOT_OWNER: &str = "Endpoint can only be called by owner";
pub const ERR_FILL_BAD_NONCE: &str =
    "The nonce you are trying to fill the SC with is not the one expected";
pub const ERR_FILL_BAD_IDENTIFIER: &str =
    "The identifier of the token you are trying to fill is not the one expected";
pub const ERR_BAD_AMOUNT_SENT: &str = "Unrecognized amount of eGLD sent.";

pub const ERR_BUY_NOT_EGLD: &str = "Sorry, the payment is not in eGLD.";
pub const ERR_SOLD_OUT: &str = "Sorry, all the eggs has been sold.";
pub const ERR_SALE_CLOSED: &str = "Sorry, the sale is closed.";
pub const ERR_SALE_NOT_OPEN: &str = "Sorry, the sale is not open.";

#[elrond_wasm::derive::contract]
pub trait PublicSaleMint: whitelist::WhitelistModule {
    #[view]
    #[storage_mapper("max_per_wallet")]
    fn max_per_wallet(&self) -> SingleValueMapper<u64>;

    #[view]
    #[storage_mapper("price_per_egg")]
    fn price_per_egg(&self) -> VecMapper<BigUint>;

    #[view]
    #[storage_mapper("reduced_price_per_egg")]
    fn reduced_price_per_egg(&self) -> VecMapper<BigUint>;

    #[storage_mapper("token_identifier")]
    fn token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("token_nonce")]
    fn token_nonce(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("already_bought")]
    fn already_bought(&self) -> MapMapper<ManagedAddress, u64>;

    #[view]
    #[storage_mapper("timestamp_sale_closed")]
    fn timestamp_sale_closed(&self) -> SingleValueMapper<u64>;

    #[init]
    fn init(
        &self,
        max_per_wallet: u64,
        price_per_egg: ManagedVec<BigUint>,
        reduced_price_per_egg: ManagedVec<BigUint>,
        timestamp_public_sale: u64,
        second_whitelist_delta: u64,
        first_whitelist_delta: u64,
        token: TokenIdentifier,
        token_nonce: u64,
        sale_duration: u64,
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
            second_whitelist_delta <= first_whitelist_delta,
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
        self.timestamp_sale_closed()
            .set(timestamp_public_sale + sale_duration);
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

    #[view(getRemainingNft)]
    fn get_remaining_nft(&self) -> BigUint {
        return self
            .blockchain()
            .get_sc_balance(&self.token_identifier().get(), self.token_nonce().get());
    }

    #[endpoint]
    #[payable("*")]
    fn buy(
        &self,
        #[payment] payment_amount: BigUint,
        #[payment_token] token: TokenIdentifier,
        #[payment_nonce] _nonce: u64,
        to_buy: u64,
    ) {
        let caller = self.blockchain().get_caller();

        if caller != self.blockchain().get_owner_address() {
            require!(self.is_sale_over() == false, ERR_SALE_CLOSED);
            require!(self.has_access(&caller) == true, ERR_SALE_NOT_OPEN);
        }
        require!(token.is_egld(), ERR_BUY_NOT_EGLD);
        require!(self.get_remaining_nft() > 0, ERR_SOLD_OUT);

        let already_bought = self.get_bought_amount(&caller);

        require!(
            self.is_price_valid(
                payment_amount,
                already_bought,
                self.get_price_list(&caller),
                to_buy
            ) == true,
            ERR_BAD_AMOUNT_SENT
        );

        // send eggs to the caller
        self.send().direct(
            &caller,
            &self.token_identifier().get(),
            self.token_nonce().get(),
            &BigUint::from(to_buy),
            &[],
        );

        self.already_bought()
            .insert(caller, already_bought + to_buy);
    }

    fn is_price_valid(
        &self,
        payment_amount: BigUint,
        already_bought: u64,
        prices: VecMapper<BigUint>,
        to_buy: u64,
    ) -> bool {
        let price_index = (to_buy + already_bought) as usize;
        if price_index == 0 || price_index > prices.len() {
            return false;
        }
        let price = prices.get(price_index);
        self.print().print_biguint(&price);

        return &price * to_buy == payment_amount;
    }

    fn is_sale_over(&self) -> bool {
        let now = self.blockchain().get_block_timestamp();
        let close = self.timestamp_sale_closed().get();

        return now >= close;
    }

    fn get_price_list(&self, address: &ManagedAddress) -> VecMapper<BigUint> {
        if self.check_contains_second(address) {
            return self.reduced_price_per_egg();
        } else {
            return self.price_per_egg();
        }
    }

    #[view(getBoughtAmount)]
    fn get_bought_amount(&self, address: &ManagedAddress) -> u64 {
        match self.already_bought().get(address) {
            Some(amount) => amount,
            None => 0,
        }
    }

    #[view(getAllBuyers)]
    fn get_all_buyers(&self) -> MultiValueEncoded<MultiValue2<ManagedAddress, u64>> {
        let mut buyers = MultiValueEncoded::new();

        for (address, balance) in self.already_bought().iter() {
            let value = MultiValue2::from((address, balance));
            buyers.push(value);
        }

        return buyers;
    }

    #[only_owner]
    #[endpoint]
    fn claim_balance(&self) {
        self.blockchain().check_caller_is_owner();

        let balance = self
            .blockchain()
            .get_sc_balance(&TokenIdentifier::egld(), 0);

        // STEP 2 : require balance > 0
        require!(
            balance > 0,
            "There is nothing to claim. The balance is empty."
        );

        // STEP 3 : send balance to owner
        let owner = self.blockchain().get_owner_address();
        self.send().direct_egld(&owner, &balance, &[]);
    }

    #[only_owner]
    #[endpoint]
    fn claim_eggs(&self) {
        self.blockchain().check_caller_is_owner();

        let balance = self
            .blockchain()
            .get_sc_balance(&self.token_identifier().get(), self.token_nonce().get());

        // STEP 2 : require balance > 0
        require!(
            balance > 0,
            "There is nothing to claim. The balance is empty."
        );

        // STEP 3 : send balance to owner
        let owner = self.blockchain().get_owner_address();
        self.send().direct(
            &owner,
            &self.token_identifier().get(),
            self.token_nonce().get(),
            &balance,
            &[],
        );
    }
}
