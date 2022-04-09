mod contract_setup;

use contract_setup::{big_uint_conv_num, setup_contract};
use elrond_wasm_debug::DebugApi;
use public_sale_mint::PublicSaleMint;

#[test]
fn buy_one_from_zero() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    let price = setup.get_price(0);

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let count = sc.calculate_buyable_eggs_count(price, 0, sc.price_per_egg());
            assert_eq!(count, 1);
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
            let p1 = sc.price_per_egg().get(1);
            let p2 = sc.price_per_egg().get(2);

            let count = sc.calculate_buyable_eggs_count(p1 + p2, 0, sc.price_per_egg());
            assert_eq!(count, 2);
        })
        .assert_ok();
}

#[test]
fn buy_two_from_one() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let p1 = sc.price_per_egg().get(2);
            let p2 = sc.price_per_egg().get(3);

            let count = sc.calculate_buyable_eggs_count(p1 + p2, 1, sc.price_per_egg());
            assert_eq!(count, 2);
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

            let _ = sc.calculate_buyable_eggs_count(p1 + delta, 1, sc.price_per_egg());
        })
        .assert_user_error(public_sale_mint::ERR_EGLD_BETWEEN_PRICE);
}

#[test]
fn send_too_much() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let p1 = big_uint_conv_num(999999);

            let _ = sc.calculate_buyable_eggs_count(p1, 1, sc.price_per_egg());
        })
        .assert_user_error(public_sale_mint::ERR_TOO_MUCH_EGLD_SENT);
}
