mod contract_setup;

use contract_setup::setup_contract;

#[test]
fn fill_by_owner() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    setup.set_eggs(&setup.owner_address.clone(), 5u64);
    setup
        .fill_eggs_from(&setup.owner_address.clone(), 5u64)
        .assert_ok();
}
