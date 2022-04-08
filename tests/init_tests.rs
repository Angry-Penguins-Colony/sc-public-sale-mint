use elrond_wasm::types::ManagedVec;
use elrond_wasm_debug::{
    testing_framework::BlockchainStateWrapper,
    tx_mock::{TxContextRef, TxResult},
};
use public_sale_mint::PublicSaleMint;
mod contract_setup;

#[test]
fn init() {
    warmup_init(|sc| {
        sc.init(
            5,
            ManagedVec::from(vec![1u64, 2u64, 3u64, 4u64]),
            ManagedVec::from(vec![1u64, 2u64, 3u64, 4u64]),
            50,
            10,
            10,
        );

        panic!("Assert eq values of sc");
    })
    .assert_ok();
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
