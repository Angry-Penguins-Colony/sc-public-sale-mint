mod contract_setup;

use contract_setup::{big_uint_conv_num, setup_contract};
use elrond_wasm::types::BigUint;
use elrond_wasm_debug::DebugApi;
use public_sale_mint::PublicSaleMint;

#[test]
fn buy_one_from_zero() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let price = sc.price_per_egg().get(1);
            let is_valid = sc.is_price_valid(price, 0, sc.price_per_egg(), 1);
            assert_eq!(is_valid, true);
        })
        .assert_ok();
}

#[test]
fn buy_two_from_zero() {
    DebugApi::dummy();

    let mut setup = setup_contract(public_sale_mint::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let p2 = sc.price_per_egg().get(2);

            let is_valid = sc.is_price_valid(&p2 + &p2, 0, sc.price_per_egg(), 2);
            assert_eq!(is_valid, true);
        })
        .assert_ok();
}

#[test]
fn buy_two_from_one() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let p3 = sc.price_per_egg().get(3);

            let is_valid = sc.is_price_valid(&p3 + &p3, 1, sc.price_per_egg(), 2);
            assert_eq!(is_valid, true);
        })
        .assert_ok();
}

#[test]
fn buy_bad_price_from_one() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let p1 = sc.price_per_egg().get(1);

            let is_valid = sc.is_price_valid(&p1 + &p1, 1, sc.price_per_egg(), 2);
            assert_eq!(is_valid, false);
        })
        .assert_ok();
}

#[test]
fn buy_with_price_not_listed() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let p1 = sc.price_per_egg().get(2);
            let delta = big_uint_conv_num(1);

            let _ = sc.is_price_valid(p1 + delta, 1, sc.price_per_egg(), 1);
        })
        .assert_ok();
}

#[test]
fn send_too_much() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let p1 = big_uint_conv_num(999999);

            let _ = sc.is_price_valid(p1, 1, sc.price_per_egg(), 1);
        })
        .assert_ok();
}

#[test]
fn buy_to_max_wallet_from_zero() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let price = sc.price_per_egg().get(5);

            let is_valid = sc.is_price_valid(
                &price + &price + &price + &price + &price,
                0,
                sc.price_per_egg(),
                5,
            );
            assert_eq!(is_valid, true);
        })
        .assert_ok();
}

#[test]
fn buy_to_max_wallet_from_max_wallet() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let mut price_sum = BigUint::<DebugApi>::zero();

            for price in sc.price_per_egg().iter() {
                price_sum += price;
            }

            let is_valid =
                sc.is_price_valid(price_sum, sc.max_per_wallet().get(), sc.price_per_egg(), 5);
            assert_eq!(is_valid, false);
        })
        .assert_ok();
}

#[test]
fn buy_one_from_max_wallet() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let price = sc.price_per_egg().get(1);

            let is_valid =
                sc.is_price_valid(price, sc.max_per_wallet().get(), sc.price_per_egg(), 1);
            assert_eq!(is_valid, false);
        })
        .assert_ok();
}
