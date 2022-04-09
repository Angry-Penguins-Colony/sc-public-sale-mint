mod contract_setup;

// use contract_setup::setup_contract;
// use elrond_wasm_debug::rust_biguint;

// #[test]
// fn buy_one_full_price() {
//     let mut setup = setup_contract(public_sale_mint::contract_obj);
//     let user = &setup.users[0].clone();

//     setup.fill_eggs(10u64);
//     setup.buy(user, &rust_biguint!(10u64)).assert_ok();

//     assert_eq!(setup.get_eggs_balance(user), rust_biguint!(1u64));
// }

// #[test]
// fn buy_two_full_price() {
//     let mut setup = setup_contract(public_sale_mint::contract_obj);
//     let user = &setup.users[0].clone();

//     setup.fill_eggs(10u64);
//     setup.buy(user, &rust_biguint!(10u64 + 9u64)).assert_ok();

//     assert_eq!(setup.get_eggs_balance(user), rust_biguint!(2u64));
// }
