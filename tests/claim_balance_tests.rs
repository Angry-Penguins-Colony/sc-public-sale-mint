mod contract_setup;

use contract_setup::setup_contract;
use elrond_wasm_debug::rust_biguint;

#[test]
fn claim_balance_while_not_owner() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    setup
        .claim_balance(&setup.users[1].clone())
        .assert_user_error(public_sale_mint::ERR_NOT_OWNER);
}

#[test]
fn claim_balance_while_owner() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    let buyer = &setup.users[0].clone();

    setup.open_public_sale();
    setup.fill_eggs(10u64);
    setup.buy(buyer, &rust_biguint!(9u64 + 9u64)).assert_ok();

    setup
        .claim_balance(&setup.owner_address.clone())
        .assert_ok();

    assert_eq!(
        setup
            .blockchain_wrapper
            .get_egld_balance(&setup.owner_address),
        rust_biguint!(9u64 + 9u64)
    );
}
