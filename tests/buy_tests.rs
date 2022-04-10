mod contract_setup;

use contract_setup::setup_contract;
use elrond_wasm_debug::rust_biguint;

enum WhitelistState {
    NotStarted,
    FirstOpen,
    SecondOpen,
    PublicOpen,
    Close,
}

enum UserWhitelist {
    None,
    First,
    Second,
}

fn buy_one(
    whitelist_state: WhitelistState,
    user_whitelist: UserWhitelist,
    amount: u64,
    expected_err: &str,
    expected_egg_balance: u64,
) {
    let mut setup = setup_contract(public_sale_mint::contract_obj);
    let user = &setup.users[0].clone();

    match whitelist_state {
        WhitelistState::NotStarted => setup.set_sale_as_not_started(),
        WhitelistState::FirstOpen => setup.open_first_whitelist(),
        WhitelistState::SecondOpen => setup.open_second_whitelist(),
        WhitelistState::PublicOpen => setup.open_public_sale(),
        WhitelistState::Close => setup.close_sale(),
    }

    match user_whitelist {
        UserWhitelist::None => (),
        UserWhitelist::First => setup.add_to_first_whitelist(user).assert_ok(),
        UserWhitelist::Second => setup.add_to_second_whitelist(user).assert_ok(),
    }

    setup.fill_eggs(10u64);
    let result = setup.buy(user, &rust_biguint!(amount));

    match expected_err {
        "" => result.assert_ok(),
        _ => result.assert_user_error(expected_err),
    }

    assert_eq!(
        setup.get_eggs_balance(user),
        rust_biguint!(expected_egg_balance)
    );
}

#[test]
fn buy_one_not_whitelisted() {
    buy_one(
        WhitelistState::NotStarted,
        UserWhitelist::None,
        10u64,
        public_sale_mint::ERR_SALE_NOT_OPEN,
        0u64,
    );

    buy_one(
        WhitelistState::FirstOpen,
        UserWhitelist::None,
        10u64,
        public_sale_mint::ERR_SALE_NOT_OPEN,
        0u64,
    );

    buy_one(
        WhitelistState::SecondOpen,
        UserWhitelist::None,
        10u64,
        public_sale_mint::ERR_SALE_NOT_OPEN,
        0u64,
    );

    buy_one(
        WhitelistState::PublicOpen,
        UserWhitelist::None,
        10u64,
        "",
        1u64,
    );

    buy_one(
        WhitelistState::Close,
        UserWhitelist::None,
        10u64,
        public_sale_mint::ERR_SALE_CLOSED,
        0u64,
    );
}

#[test]
fn buy_one_first_whitelisted() {
    buy_one(
        WhitelistState::NotStarted,
        UserWhitelist::First,
        10u64,
        public_sale_mint::ERR_SALE_NOT_OPEN,
        0u64,
    );

    buy_one(
        WhitelistState::FirstOpen,
        UserWhitelist::First,
        10u64,
        "",
        1u64,
    );

    buy_one(
        WhitelistState::SecondOpen,
        UserWhitelist::First,
        10u64,
        "",
        1u64,
    );

    buy_one(
        WhitelistState::PublicOpen,
        UserWhitelist::First,
        10u64,
        "",
        1u64,
    );

    buy_one(
        WhitelistState::Close,
        UserWhitelist::First,
        10u64,
        public_sale_mint::ERR_SALE_CLOSED,
        0u64,
    );
}
