mod contract_setup;

use contract_setup::setup_contract;
use elrond_wasm::types::ManagedAddress;
use elrond_wasm_debug::rust_biguint;
use public_sale_mint::whitelist::WhitelistModule;

#[test]
fn add_to_first_whitelisted() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    let user = setup.users[0].clone();
    setup.add_to_first_whitelist(user.clone()).assert_ok();

    setup
        .blockchain_wrapper
        .execute_tx(&user, &setup.contract_wrapper, &rust_biguint!(0u64), |sc| {
            let contains = sc.check_contains_first(ManagedAddress::from_address(&user));
            assert_eq!(contains, true);
        })
        .assert_ok();
}

#[test]
fn add_to_second_whitelisted() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    let user = setup.users[0].clone();
    setup.add_to_second_whitelist(user.clone()).assert_ok();

    setup
        .blockchain_wrapper
        .execute_tx(&user, &setup.contract_wrapper, &rust_biguint!(0u64), |sc| {
            let contains = sc.check_contains_second(ManagedAddress::from_address(&user));
            assert_eq!(contains, true);
        })
        .assert_ok();
}

#[test]
fn normal_user_adding_to_first_wl() {
    let setup = setup_contract(public_sale_mint::contract_obj);

    let mut blockchain_state_wrapper = setup.blockchain_wrapper;
    let address = &setup.users[0].clone();

    blockchain_state_wrapper
        .execute_tx(
            address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.add_to_first_whitelist(ManagedAddress::from_address(&address));
            },
        )
        .assert_user_error(public_sale_mint::whitelist::ERR_NOT_OWNER);
}

#[test]
fn normal_user_adding_to_second_wl() {
    let setup = setup_contract(public_sale_mint::contract_obj);

    let mut blockchain_state_wrapper = setup.blockchain_wrapper;
    let address = &setup.users[0].clone();

    blockchain_state_wrapper
        .execute_tx(
            address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.add_to_second_whitelist(ManagedAddress::from_address(&address));
            },
        )
        .assert_user_error(public_sale_mint::whitelist::ERR_NOT_OWNER);
}

#[test]
fn check_contains_first_on_not_whitelisted() {
    let setup = setup_contract(public_sale_mint::contract_obj);

    let mut blockchain_state_wrapper = setup.blockchain_wrapper;
    let address = &setup.users[0].clone();

    blockchain_state_wrapper
        .execute_tx(
            address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                let contains = sc.check_contains_first(ManagedAddress::from_address(&address));
                assert_eq!(contains, false);
            },
        )
        .assert_ok();
}

#[test]
fn check_contains_second_on_not_whitelisted() {
    let setup = setup_contract(public_sale_mint::contract_obj);

    let mut blockchain_state_wrapper = setup.blockchain_wrapper;
    let address = &setup.users[0].clone();

    blockchain_state_wrapper
        .execute_tx(
            address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                let contains = sc.check_contains_second(ManagedAddress::from_address(&address));
                assert_eq!(contains, false);
            },
        )
        .assert_ok();
}