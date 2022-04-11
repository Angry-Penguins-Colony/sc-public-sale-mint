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

fn buy(
    whitelist_state: &WhitelistState,
    user_whitelist: &UserWhitelist,
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

    assert_eq!(setup.get_buyed_amount(user), expected_egg_balance);
}

#[test]
fn buy_one_not_whitelisted() {
    let user_whitelist = &&UserWhitelist::None;
    buy(
        &WhitelistState::NotStarted,
        user_whitelist,
        10u64,
        public_sale_mint::ERR_SALE_NOT_OPEN,
        0u64,
    );

    buy(
        &WhitelistState::FirstOpen,
        user_whitelist,
        10u64,
        public_sale_mint::ERR_SALE_NOT_OPEN,
        0u64,
    );

    buy(
        &WhitelistState::SecondOpen,
        user_whitelist,
        10u64,
        public_sale_mint::ERR_SALE_NOT_OPEN,
        0u64,
    );

    buy(&WhitelistState::PublicOpen, user_whitelist, 10u64, "", 1u64);

    buy(
        &WhitelistState::Close,
        user_whitelist,
        10u64,
        public_sale_mint::ERR_SALE_CLOSED,
        0u64,
    );
}

#[test]
fn buy_one_first_whitelisted() {
    let user_whitelist = &&UserWhitelist::First;

    buy(
        &WhitelistState::NotStarted,
        user_whitelist,
        10u64,
        public_sale_mint::ERR_SALE_NOT_OPEN,
        0u64,
    );

    buy(&WhitelistState::FirstOpen, user_whitelist, 10u64, "", 1u64);
    buy(&WhitelistState::SecondOpen, user_whitelist, 10u64, "", 1u64);
    buy(&WhitelistState::PublicOpen, user_whitelist, 10u64, "", 1u64);

    buy(
        &WhitelistState::Close,
        user_whitelist,
        10u64,
        public_sale_mint::ERR_SALE_CLOSED,
        0u64,
    );
}

#[test]
fn buy_one_second_whitelisted() {
    let whitelisted = &UserWhitelist::Second;

    buy(
        &WhitelistState::NotStarted,
        whitelisted,
        5u64,
        public_sale_mint::ERR_SALE_NOT_OPEN,
        0u64,
    );

    buy(
        &WhitelistState::FirstOpen,
        whitelisted,
        5u64,
        public_sale_mint::ERR_SALE_NOT_OPEN,
        0u64,
    );
    buy(&WhitelistState::SecondOpen, whitelisted, 5u64, "", 1u64);
    buy(&WhitelistState::PublicOpen, whitelisted, 5u64, "", 1u64);

    buy(
        &WhitelistState::Close,
        whitelisted,
        5u64,
        public_sale_mint::ERR_SALE_CLOSED,
        0u64,
    );
}

#[test]
fn buy_two_not_whitelisted() {
    let user_whitelist = &&UserWhitelist::None;
    buy(
        &WhitelistState::NotStarted,
        user_whitelist,
        18u64,
        public_sale_mint::ERR_SALE_NOT_OPEN,
        0u64,
    );

    buy(
        &WhitelistState::FirstOpen,
        user_whitelist,
        18u64,
        public_sale_mint::ERR_SALE_NOT_OPEN,
        0u64,
    );

    buy(
        &WhitelistState::SecondOpen,
        user_whitelist,
        18u64,
        public_sale_mint::ERR_SALE_NOT_OPEN,
        0u64,
    );

    buy(&WhitelistState::PublicOpen, user_whitelist, 18u64, "", 2u64);

    buy(
        &WhitelistState::Close,
        user_whitelist,
        18u64,
        public_sale_mint::ERR_SALE_CLOSED,
        0u64,
    );
}

#[test]
fn buy_two_first_whitelisted() {
    let user_whitelist = &&UserWhitelist::First;

    buy(
        &WhitelistState::NotStarted,
        user_whitelist,
        18u64,
        public_sale_mint::ERR_SALE_NOT_OPEN,
        0u64,
    );

    buy(&WhitelistState::FirstOpen, user_whitelist, 18u64, "", 2u64);
    buy(&WhitelistState::SecondOpen, user_whitelist, 18u64, "", 2u64);
    buy(&WhitelistState::PublicOpen, user_whitelist, 18u64, "", 2u64);

    buy(
        &WhitelistState::Close,
        user_whitelist,
        18u64,
        public_sale_mint::ERR_SALE_CLOSED,
        0u64,
    );
}

#[test]
fn buy_two_second_whitelisted() {
    let whitelisted = &UserWhitelist::Second;

    buy(
        &WhitelistState::NotStarted,
        whitelisted,
        8u64,
        public_sale_mint::ERR_SALE_NOT_OPEN,
        0u64,
    );

    buy(
        &WhitelistState::FirstOpen,
        whitelisted,
        8u64,
        public_sale_mint::ERR_SALE_NOT_OPEN,
        0u64,
    );
    buy(&WhitelistState::SecondOpen, whitelisted, 8u64, "", 2u64);
    buy(&WhitelistState::PublicOpen, whitelisted, 8u64, "", 2u64);

    buy(
        &WhitelistState::Close,
        whitelisted,
        8u64,
        public_sale_mint::ERR_SALE_CLOSED,
        0u64,
    );
}
