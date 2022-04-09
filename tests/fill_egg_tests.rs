mod contract_setup;

use contract_setup::setup_contract;
use elrond_wasm::contract_base::ContractBase;
use elrond_wasm_debug::rust_biguint;
use public_sale_mint::PublicSaleMint;

#[test]
fn fill_by_owner() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    setup.set_eggs(&setup.owner_address.clone(), 5u64);
    setup
        .fill_eggs_from(&setup.owner_address.clone(), 5u64)
        .assert_ok();
}

#[test]
fn fill_by_non_owner() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);

    let user = setup.users[0].clone();

    setup.set_eggs(&user, 5u64);
    setup
        .fill_eggs_from(&user, 5u64)
        .assert_user_error(public_sale_mint::ERR_NOT_OWNER);
}

#[test]
fn fill_bad_nonce() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let owner = &setup.owner_address.clone();
    const BAD_NONCE: u64 = 5u64;

    assert_ne!(
        setup.egg_nonce, BAD_NONCE,
        "You should change BAD_NONCE to be different from egg_nonce. =)"
    );

    setup.blockchain_wrapper.set_nft_balance(
        owner,
        &setup.egg_id,
        BAD_NONCE,
        &rust_biguint!(1u64),
        &{},
    );

    setup
        .blockchain_wrapper
        .execute_esdt_transfer(
            owner,
            &setup.contract_wrapper,
            &setup.egg_id,
            BAD_NONCE,
            &rust_biguint!(1u64),
            |sc| {
                let payment = sc.call_value().payment_as_tuple();

                sc.fill_egg(payment.2, payment.0, payment.1);
            },
        )
        .assert_user_error(public_sale_mint::ERR_FILL_BAD_NONCE);
}

#[test]
fn fill_bad_token() {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let owner = &setup.owner_address.clone();
    const BAD_TOKEN: &[u8] = b"HEENOK";

    assert_ne!(
        setup.egg_id, BAD_TOKEN,
        "You should change BAD_TOKEN to be different from egg_nonce. =)"
    );

    setup.blockchain_wrapper.set_nft_balance(
        owner,
        &BAD_TOKEN,
        setup.egg_nonce,
        &rust_biguint!(1u64),
        &{},
    );

    setup
        .blockchain_wrapper
        .execute_esdt_transfer(
            owner,
            &setup.contract_wrapper,
            &BAD_TOKEN,
            setup.egg_nonce,
            &rust_biguint!(1u64),
            |sc| {
                let payment = sc.call_value().payment_as_tuple();

                sc.fill_egg(payment.2, payment.0, payment.1);
            },
        )
        .assert_user_error(public_sale_mint::ERR_FILL_BAD_IDENTIFIER);
}
