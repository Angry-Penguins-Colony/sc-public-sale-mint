mod contract_setup;

use contract_setup::setup_contract;
use elrond_wasm_debug::rust_biguint;
use public_sale_mint::PublicSaleMint;

#[test]
fn claim_balance_while_not_owner() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.users[0].clone(),
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.claim_eggs();
            },
        )
        .assert_user_error(public_sale_mint::ERR_NOT_OWNER);
}

#[test]
fn claim_balance_while_owner() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    setup.fill_eggs(10u64);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address.clone(),
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.claim_eggs();
            },
        )
        .assert_ok();

    assert_eq!(
        setup.get_eggs_balance(&setup.owner_address.clone()),
        rust_biguint!(10u64)
    );
}
