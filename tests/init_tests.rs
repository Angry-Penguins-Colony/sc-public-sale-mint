use elrond_wasm::types::{ManagedVec, TokenIdentifier};
use elrond_wasm_debug::{
    testing_framework::BlockchainStateWrapper,
    tx_mock::{TxContextRef, TxResult},
};
use public_sale_mint::{whitelist::WhitelistModule, PublicSaleMint};
mod contract_setup;

#[test]
fn init() {
    warmup_init(|sc| {
        sc.init(
            3,
            ManagedVec::from(vec![1u64, 5u64, 10u64]),
            ManagedVec::from(vec![1u64, 4u64, 9u64]),
            50,
            10,
            20,
            TokenIdentifier::from_esdt_bytes(b"TOKEN"),
            3,
            10,
        );

        assert_eq!(sc.max_per_wallet().get(), 3);
        assert_eq!(sc.price_per_egg().len(), 3);
        assert_eq!(sc.price_per_egg().get(1), 1u64);
        assert_eq!(sc.price_per_egg().get(2), 5u64);
        assert_eq!(sc.price_per_egg().get(3), 10u64);
        assert_eq!(sc.reduced_price_per_egg().len(), 3);
        assert_eq!(sc.reduced_price_per_egg().get(1), 1u64);
        assert_eq!(sc.reduced_price_per_egg().get(2), 4u64);
        assert_eq!(sc.reduced_price_per_egg().get(3), 9u64);
        assert_eq!(sc.timestamp_public_sale().get(), 50);
        assert_eq!(sc.timestamp_second_whitelist().get(), 40);
        assert_eq!(sc.timestamp_first_whitelist().get(), 30);
        assert_eq!(
            sc.token_identifier().get(),
            TokenIdentifier::from_esdt_bytes(b"TOKEN")
        );
        assert_eq!(sc.token_nonce().get(), 3);
        assert_eq!(sc.timestamp_sale_closed().get(), 60);
    })
    .assert_ok();
}

#[test]
fn init_second_wl_lesser_then_first() {
    warmup_init(|sc| {
        sc.init(
            5,
            ManagedVec::from(vec![1u64, 2u64, 3u64, 4u64, 5u64]),
            ManagedVec::from(vec![1u64, 2u64, 3u64, 4u64, 5u64]),
            0,
            5,
            0,
            TokenIdentifier::from_esdt_bytes(b"TOKEN"),
            3,
            0,
        );
    })
    .assert_user_error(public_sale_mint::ERR_INIT_SECOND_WL_LESSER_THEN_FIRST);
}

#[test]
fn full_prices_length_different_from_max_per_wallet() {
    warmup_init(|sc| {
        sc.init(
            5,
            ManagedVec::from(vec![1u64, 2u64, 3u64, 4u64]),
            ManagedVec::from(vec![1u64, 2u64, 3u64, 4u64, 5u64]),
            0,
            0,
            0,
            TokenIdentifier::from_esdt_bytes(b"TOKEN"),
            3,
            0,
        );
    })
    .assert_user_error(public_sale_mint::ERR_INIT_PRICE_PER_EGG_DIFF);
}

#[test]
fn reduced_prices_length_different_from_max_per_wallet() {
    warmup_init(|sc| {
        sc.init(
            5,
            ManagedVec::from(vec![1u64, 2u64, 3u64, 4u64, 5u64]),
            ManagedVec::from(vec![1u64, 2u64, 3u64, 4u64]),
            0,
            0,
            0,
            TokenIdentifier::from_esdt_bytes(b"TOKEN"),
            3,
            0,
        );
    })
    .assert_user_error(public_sale_mint::ERR_INIT_REDUCED_PRICE_PER_EGG_DIFF);
}

#[test]
fn full_price_equals_0() {
    warmup_init(|sc| {
        sc.init(
            5,
            ManagedVec::new(),
            ManagedVec::from(vec![1u64, 2u64, 3u64, 4u64, 5u64]),
            0,
            0,
            0,
            TokenIdentifier::from_esdt_bytes(b"TOKEN"),
            3,
            0,
        );
    })
    .assert_user_error(public_sale_mint::ERR_INIT_PRICE_PER_EGG_ZERO);
}

#[test]
fn reduced_price_equals_0() {
    warmup_init(|sc| {
        sc.init(
            5,
            ManagedVec::from(vec![1u64, 2u64, 3u64, 4u64]),
            ManagedVec::new(),
            0,
            0,
            0,
            TokenIdentifier::from_esdt_bytes(b"TOKEN"),
            3,
            0,
        );
    })
    .assert_user_error(public_sale_mint::ERR_INIT_REDUCED_PRICE_PER_EGG_ZERO);
}

pub fn warmup_init(tx_fn: fn(public_sale_mint::ContractObj<TxContextRef>)) -> TxResult {
    let rust_zero = elrond_wasm_debug::rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_zero);
    let cf_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        public_sale_mint::contract_obj,
        contract_setup::WASM_PATH,
    );

    return blockchain_wrapper.execute_tx(&owner_address, &cf_wrapper, &rust_zero, tx_fn);
}
