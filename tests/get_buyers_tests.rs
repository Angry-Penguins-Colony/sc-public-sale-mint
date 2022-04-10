mod contract_setup;

use contract_setup::setup_contract;
use elrond_wasm::types::ManagedAddress;
use elrond_wasm_debug::{rust_biguint, DebugApi};
use public_sale_mint::PublicSaleMint;

#[test]
fn get_buyers_should_be_empty() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let buyers = sc.get_all_buyers();
            let mut buyers_iter = buyers.into_iter();

            assert_eq!(buyers_iter.next().is_none(), true);
        })
        .assert_ok();
}

#[test]
fn get_buyers_should_contains_one_with_one() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    setup.open_public_sale();
    setup.fill_eggs(10u64);
    setup.buy(user, &rust_biguint!(10u64)).assert_ok();

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let buyers = sc.get_all_buyers();
            let mut buyers_iter = buyers.into_iter();

            let next = buyers_iter.next();

            assert_eq!(
                next.unwrap().into_tuple(),
                (ManagedAddress::<DebugApi>::from_address(user), 1)
            );

            assert_eq!(buyers_iter.next().is_none(), true);
        })
        .assert_ok();
}

#[test]
fn get_buyers_should_contains_one_with_two() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    setup.open_public_sale();
    setup.fill_eggs(10u64);
    setup.buy(user, &rust_biguint!(10u64)).assert_ok();
    setup.buy(user, &rust_biguint!(9u64)).assert_ok();

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let buyers = sc.get_all_buyers();
            let mut buyers_iter = buyers.into_iter();

            let next = buyers_iter.next();

            assert_eq!(
                next.unwrap().into_tuple(),
                (ManagedAddress::<DebugApi>::from_address(user), 2)
            );

            assert_eq!(buyers_iter.next().is_none(), true);
        })
        .assert_ok();
}

#[test]
fn get_buyers_should_contains_two_users() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user1 = &setup.users[0].clone();
    let user2 = &setup.users[1].clone();

    setup.open_public_sale();
    setup.fill_eggs(10u64);
    setup.buy(user1, &rust_biguint!(10u64)).assert_ok();
    setup.buy(user2, &rust_biguint!(10u64)).assert_ok();

    setup
        .blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let buyers = sc.get_all_buyers();
            let mut buyers_iter = buyers.into_iter();

            assert_eq!(
                buyers_iter.next().unwrap().into_tuple(),
                (ManagedAddress::<DebugApi>::from_address(user1), 1)
            );

            assert_eq!(
                buyers_iter.next().unwrap().into_tuple(),
                (ManagedAddress::<DebugApi>::from_address(user2), 1)
            );

            assert_eq!(buyers_iter.next().is_none(), true);
        })
        .assert_ok();
}
