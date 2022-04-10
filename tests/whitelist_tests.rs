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
            let contains = sc.check_contains_first(&ManagedAddress::from_address(&user));
            assert_eq!(contains, true);
        })
        .assert_ok();
}

#[test]
fn add_to_second_whitelisted() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    let user = setup.users[0].clone();
    setup.add_to_second_whitelist(&user.clone()).assert_ok();

    setup
        .blockchain_wrapper
        .execute_tx(&user, &setup.contract_wrapper, &rust_biguint!(0u64), |sc| {
            let contains = sc.check_contains_second(&ManagedAddress::from_address(&user));
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
                sc.add_to_first_whitelist(&ManagedAddress::from_address(&address));
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
                sc.add_to_second_whitelist(&ManagedAddress::from_address(&address));
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
                let contains = sc.check_contains_first(&ManagedAddress::from_address(&address));
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
                let contains = sc.check_contains_second(&ManagedAddress::from_address(&address));
                assert_eq!(contains, false);
            },
        )
        .assert_ok();
}

#[test]
fn remove_first_whitelist() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    let user = setup.users[0].clone();
    setup.add_to_first_whitelist(user.clone()).assert_ok();
    setup.remove_from_first_whitelist(user.clone()).assert_ok();
    assert_eq!(setup.is_first_whitelisted(user.clone()), false);
}

#[test]
fn remove_from_second_whitelist() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    let user = setup.users[0].clone();
    setup.add_to_second_whitelist(&user.clone()).assert_ok();
    setup.remove_from_second_whitelist(user.clone()).assert_ok();
    assert_eq!(setup.is_second_whitelisted(user.clone()), false);
}

#[test]
fn remove_first_whitelist_while_not_owner() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    let address = &setup.users[0].clone();

    setup.add_to_first_whitelist(address.clone()).assert_ok();

    setup
        .blockchain_wrapper
        .execute_tx(
            address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.remove_from_first_whitelist(&ManagedAddress::from_address(&address));
            },
        )
        .assert_user_error(public_sale_mint::whitelist::ERR_NOT_OWNER);
}

#[test]
fn remove_second_whitelist_while_not_owner() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    let address = &setup.users[0].clone();

    setup.add_to_second_whitelist(&address.clone()).assert_ok();

    setup
        .blockchain_wrapper
        .execute_tx(
            address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.remove_from_second_whitelist(&ManagedAddress::from_address(&address));
            },
        )
        .assert_user_error(public_sale_mint::whitelist::ERR_NOT_OWNER);
}

#[test]
fn has_access_in_public_open() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    let unwhitelisted = setup.users[0].clone();
    let second_whitelisted = setup.users[1].clone();
    let first_whitelisted = setup.users[2].clone();

    setup
        .add_to_second_whitelist(&second_whitelisted.clone())
        .assert_ok();

    setup
        .add_to_first_whitelist(first_whitelisted.clone())
        .assert_ok();

    setup.open_public_sale();

    assert_eq!(setup.has_access(&unwhitelisted), true);
    assert_eq!(setup.has_access(&second_whitelisted), true);
    assert_eq!(setup.has_access(&first_whitelisted), true);
}

#[test]
fn has_access_in_second_whitelisted_open() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    let unwhitelisted = setup.users[0].clone();
    let second_whitelisted = setup.users[1].clone();
    let first_whitelisted = setup.users[2].clone();

    setup
        .add_to_second_whitelist(&second_whitelisted.clone())
        .assert_ok();

    setup
        .add_to_first_whitelist(first_whitelisted.clone())
        .assert_ok();

    setup.open_second_whitelist();

    assert_eq!(setup.has_access(&unwhitelisted), false);
    assert_eq!(setup.has_access(&second_whitelisted), true);
    assert_eq!(setup.has_access(&first_whitelisted), true);
}

#[test]
fn has_access_in_first_whitelisted_open() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    let unwhitelisted = setup.users[0].clone();
    let second_whitelisted = setup.users[1].clone();
    let first_whitelisted = setup.users[2].clone();

    setup
        .add_to_second_whitelist(&second_whitelisted.clone())
        .assert_ok();

    setup
        .add_to_first_whitelist(first_whitelisted.clone())
        .assert_ok();

    setup.open_first_whitelist();

    assert_eq!(setup.has_access(&unwhitelisted), false);
    assert_eq!(setup.has_access(&second_whitelisted), false);
    assert_eq!(setup.has_access(&first_whitelisted), true);
}

#[test]
fn has_access_in_no_whitelisted_open() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    let unwhitelisted = setup.users[0].clone();
    let second_whitelisted = setup.users[1].clone();
    let first_whitelisted = setup.users[2].clone();

    setup
        .add_to_second_whitelist(&second_whitelisted.clone())
        .assert_ok();

    setup
        .add_to_first_whitelist(first_whitelisted.clone())
        .assert_ok();

    assert_eq!(setup.has_access(&unwhitelisted), false);
    assert_eq!(setup.has_access(&second_whitelisted), false);
    assert_eq!(setup.has_access(&first_whitelisted), false);
}
