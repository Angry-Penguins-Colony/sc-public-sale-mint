mod contract_setup;

use contract_setup::setup_contract;
use elrond_wasm::contract_base::ContractBase;
use elrond_wasm_debug::rust_biguint;
use public_sale_mint::PublicSaleMint;

#[test]
fn buy_two_then_one() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    setup.open_public_sale();

    setup.fill_eggs(10u64);
    setup.buy(user, &rust_biguint!(9u64 + 9u64), 2).assert_ok();
    setup.buy(user, &rust_biguint!(8u64), 1).assert_ok();

    assert_eq!(setup.get_eggs_balance(user), rust_biguint!(3u64));
    assert_eq!(setup.get_buyed_amount(user), 3u64);
}

#[test]
fn buy_two_then_one_reduced_price() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    setup.open_public_sale();

    setup.add_to_second_whitelist(user).assert_ok();
    setup.fill_eggs(10u64);
    setup.buy(user, &rust_biguint!(4u64 + 4u64), 2).assert_ok();
    setup.buy(user, &rust_biguint!(3u64), 1).assert_ok();

    assert_eq!(setup.get_eggs_balance(user), rust_biguint!(3u64));
    assert_eq!(setup.get_buyed_amount(user), 3u64);
}

#[test]
fn buy_with_not_egld() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    const TOKEN_ID: &[u8] = b"SOME TOKEN";
    const TOKEN_NONCE: u64 = 5u64;

    setup.blockchain_wrapper.set_nft_balance(
        &user,
        TOKEN_ID,
        TOKEN_NONCE,
        &rust_biguint!(10u64),
        &{},
    );

    setup.open_public_sale();
    setup
        .blockchain_wrapper
        .execute_esdt_transfer(
            &user,
            &setup.contract_wrapper,
            TOKEN_ID,
            TOKEN_NONCE,
            &rust_biguint!(1u64),
            |sc| {
                let payment = sc.call_value().payment_as_tuple();

                sc.buy(payment.2, payment.0, payment.1, 1u64);
            },
        )
        .assert_user_error(public_sale_mint::ERR_BUY_NOT_EGLD);

    assert_eq!(setup.get_buyed_amount(user), 0);
}

#[test]
fn buy_exceed_price() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    setup.open_public_sale();
    setup.fill_eggs(10u64);
    setup
        .buy(user, &rust_biguint!(150u64), 1)
        .assert_user_error(public_sale_mint::ERR_BAD_AMOUNT_SENT);

    assert_eq!(setup.get_buyed_amount(user), 0);
}

#[test]
fn buy_exceed_price_while_reduced_price() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    setup.open_public_sale();
    setup.add_to_second_whitelist(user).assert_ok();
    setup.fill_eggs(10u64);
    setup
        .buy(user, &rust_biguint!(150u64), 1)
        .assert_user_error(public_sale_mint::ERR_BAD_AMOUNT_SENT);

    assert_eq!(setup.get_buyed_amount(user), 0);
}

#[test]
fn buy_dont_fit_price() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    setup.open_public_sale();
    setup.fill_eggs(10u64);
    setup
        .buy(user, &rust_biguint!(10u64 + 5u64), 1)
        .assert_user_error(public_sale_mint::ERR_BAD_AMOUNT_SENT);
    assert_eq!(setup.get_buyed_amount(user), 0);
}

#[test]
fn buy_dont_fit_price_while_reduced_price() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    setup.open_public_sale();
    setup.add_to_second_whitelist(user).assert_ok();
    setup.fill_eggs(10u64);
    setup
        .buy(user, &rust_biguint!(5u64 + 1u64), 1)
        .assert_user_error(public_sale_mint::ERR_BAD_AMOUNT_SENT);
    assert_eq!(setup.get_buyed_amount(user), 0);
}

#[test]
fn buy_with_zero() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    setup.open_public_sale();
    setup.fill_eggs(10u64);
    setup
        .buy(user, &rust_biguint!(0u64), 1)
        .assert_user_error(public_sale_mint::ERR_BAD_AMOUNT_SENT);
    assert_eq!(setup.get_buyed_amount(user), 0);
}

#[test]
fn buy_with_zero_reduced_price() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    setup.open_public_sale();
    setup.add_to_second_whitelist(user).assert_ok();
    setup.fill_eggs(10u64);
    setup
        .buy(user, &rust_biguint!(0u64), 1)
        .assert_user_error(public_sale_mint::ERR_BAD_AMOUNT_SENT);
    assert_eq!(setup.get_buyed_amount(user), 0);
}

#[test]
fn buy_while_no_remaining_sft() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    setup.open_public_sale();
    setup.fill_eggs(1u64);
    setup.buy(user, &rust_biguint!(10u64), 1).assert_ok();

    setup
        .buy(user, &rust_biguint!(9u64), 1)
        .assert_user_error(public_sale_mint::ERR_SOLD_OUT);
    assert_eq!(setup.get_buyed_amount(user), 1);
}

#[test]
fn buy_while_no_remaining_sft_while_reduced() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    setup.open_public_sale();
    setup.add_to_second_whitelist(user).assert_ok();
    setup.fill_eggs(1u64);
    setup.buy(user, &rust_biguint!(5u64), 1).assert_ok();

    setup
        .buy(user, &rust_biguint!(4u64), 1)
        .assert_user_error(public_sale_mint::ERR_SOLD_OUT);
    assert_eq!(setup.get_buyed_amount(user), 1);
}

#[test]
fn buy_with_full_price_while_reduced() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    setup.open_public_sale();
    setup.add_to_second_whitelist(user).assert_ok();
    setup.fill_eggs(10u64);
    setup
        .buy(user, &rust_biguint!(10u64), 1)
        .assert_user_error(public_sale_mint::ERR_BAD_AMOUNT_SENT);
    assert_eq!(setup.get_buyed_amount(user), 0);
}

#[test]
fn buy_with_reduced_price_while_full() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    setup.open_public_sale();
    setup.fill_eggs(10u64);
    setup
        .buy(user, &rust_biguint!(5u64), 1)
        .assert_user_error(public_sale_mint::ERR_BAD_AMOUNT_SENT);
    assert_eq!(setup.get_buyed_amount(user), 0);
}

#[test]
fn buy_while_closed() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    setup.open_public_sale();
    setup.fill_eggs(10u64);
    setup.close_sale();

    setup
        .buy(user, &rust_biguint!(10u64), 1)
        .assert_user_error(public_sale_mint::ERR_SALE_CLOSED);
    assert_eq!(setup.get_buyed_amount(user), 0);
}
