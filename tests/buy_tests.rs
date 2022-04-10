mod contract_setup;

use contract_setup::setup_contract;
use elrond_wasm::contract_base::ContractBase;
use elrond_wasm_debug::rust_biguint;
use public_sale_mint::PublicSaleMint;

#[test]
fn buy_one_full_price() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    setup.fill_eggs(10u64);
    setup.buy(user, &rust_biguint!(10u64)).assert_ok();

    assert_eq!(setup.get_eggs_balance(user), rust_biguint!(1u64));
}

#[test]
fn buy_two_full_price() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    setup.fill_eggs(10u64);
    setup.buy(user, &rust_biguint!(10u64 + 9u64)).assert_ok();

    assert_eq!(setup.get_eggs_balance(user), rust_biguint!(2u64));
}

#[test]
fn buy_two_then_one() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    setup.fill_eggs(10u64);
    setup.buy(user, &rust_biguint!(10u64 + 9u64)).assert_ok();
    setup.buy(user, &rust_biguint!(8u64)).assert_ok();

    assert_eq!(setup.get_eggs_balance(user), rust_biguint!(3u64));
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

                sc.buy(payment.2, payment.0, payment.1);
            },
        )
        .assert_user_error(public_sale_mint::ERR_BUY_NOT_EGLD)
}
