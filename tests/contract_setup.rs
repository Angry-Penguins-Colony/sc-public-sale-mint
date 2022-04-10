use elrond_wasm::api::{BigIntApi, ManagedTypeApi};
use elrond_wasm::elrond_codec::multi_types::MultiValue2;
use elrond_wasm::types::MultiValueEncoded;
use elrond_wasm::{
    contract_base::ContractBase,
    types::{Address, BigUint, ManagedAddress, ManagedType, ManagedVec, TokenIdentifier},
};
use elrond_wasm_debug::tx_mock::TxContextRef;
use elrond_wasm_debug::{rust_biguint, testing_framework::*, tx_mock::TxResult, DebugApi};
use public_sale_mint::{whitelist::WhitelistModule, *};

pub const WASM_PATH: &'static str = "output/empty.wasm";
pub const PUBLIC_TIMESTAMP: u64 = 120;
pub const SECOND_WHITELIST_TIMESTAMP_DELTA: u64 = 20;
pub const FIRST_WHITELIST_TIMESTAMP_DELTA: u64 = 40;
pub const SALE_DURATION: u64 = 140;
pub const EGG_ID: [u8; 3] = *b"EGG";
pub const EGG_NONCE: u64 = 1;

pub struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> public_sale_mint::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub users: [Address; 4],
    pub user_first_whitelisted: Address,
    pub user_second_whitelisted: Address,
    pub contract_wrapper:
        ContractObjWrapper<public_sale_mint::ContractObj<DebugApi>, ContractObjBuilder>,
    pub public_timestamp: u64,
    pub first_whitelist_timestamp: u64,
    pub second_whitelist_timestamp: u64,
    pub egg_id: [u8; 3],
    pub egg_nonce: u64,
    pub sale_duration: u64,
}

impl<ContractObjBuilder> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> public_sale_mint::ContractObj<DebugApi>,
{
    #[allow(dead_code)]
    pub fn get_max_per_wallet(&mut self) -> u64 {
        let mut output = None;

        self.blockchain_wrapper
            .execute_query(&self.contract_wrapper, |sc| {
                output = Some(sc.max_per_wallet().get());
            })
            .assert_ok();

        assert_eq!(output.is_some(), true, "Cannot get the price of the egg");

        return output.unwrap();
    }

    #[allow(dead_code)]
    pub fn get_price(&mut self, index: usize) -> BigUint<DebugApi> {
        let mut output = None;

        self.blockchain_wrapper
            .execute_query(&self.contract_wrapper, |sc| {
                output = Some(sc.price_per_egg().get(index + 1));

                assert_eq!(output.is_some(), true, "Cannot get the price of the egg");
            })
            .assert_ok();

        assert_eq!(output.is_some(), true, "Cannot get the price of the egg");

        return output.unwrap();
    }

    #[allow(dead_code)]
    pub fn add_to_first_whitelist(&mut self, address: &Address) -> TxResult {
        let tx_result = self.blockchain_wrapper.execute_tx(
            &self.owner_address,
            &self.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.add_to_first_whitelist(&ManagedAddress::from_address(address));
            },
        );

        return tx_result;
    }

    #[allow(dead_code)]
    pub fn add_to_second_whitelist(&mut self, address: &Address) -> TxResult {
        let tx_result = self.blockchain_wrapper.execute_tx(
            &self.owner_address,
            &self.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.add_to_second_whitelist(&ManagedAddress::from_address(address));
            },
        );

        return tx_result;
    }

    #[allow(dead_code)]
    pub fn is_first_whitelisted(&mut self, address: Address) -> bool {
        let mut output = Option::None;
        self.blockchain_wrapper
            .execute_query(&self.contract_wrapper, |sc| {
                output = Some(sc.check_contains_first(&ManagedAddress::from_address(&address)));
            })
            .assert_ok();

        return output.unwrap();
    }

    #[allow(dead_code)]
    pub fn is_second_whitelisted(&mut self, address: Address) -> bool {
        let mut output = Option::None;
        self.blockchain_wrapper
            .execute_query(&self.contract_wrapper, |sc| {
                output = Some(sc.check_contains_second(&ManagedAddress::from_address(&address)));
            })
            .assert_ok();

        return output.unwrap();
    }

    #[allow(dead_code)]
    pub fn remove_from_first_whitelist(&mut self, address: Address) -> TxResult {
        let tx_result = self.blockchain_wrapper.execute_tx(
            &self.owner_address,
            &self.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.remove_from_first_whitelist(&ManagedAddress::from_address(&address));
            },
        );

        return tx_result;
    }

    #[allow(dead_code)]
    pub fn remove_from_second_whitelist(&mut self, address: Address) -> TxResult {
        let tx_result = self.blockchain_wrapper.execute_tx(
            &self.owner_address,
            &self.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.remove_from_second_whitelist(&ManagedAddress::from_address(&address));
            },
        );

        return tx_result;
    }
    #[allow(dead_code)]
    pub fn set_sale_as_not_started(&mut self) {
        let sale_not_started_timestamp = self.first_whitelist_timestamp - 1;

        assert_eq!(
            &sale_not_started_timestamp >= &0,
            true,
            "Cannot set the sale as not started"
        );

        self.blockchain_wrapper
            .set_block_timestamp(sale_not_started_timestamp);
    }

    #[allow(dead_code)]
    pub fn open_first_whitelist(&mut self) {
        self.blockchain_wrapper
            .set_block_timestamp(self.first_whitelist_timestamp);
    }

    #[allow(dead_code)]
    pub fn open_second_whitelist(&mut self) {
        self.blockchain_wrapper
            .set_block_timestamp(self.second_whitelist_timestamp);
    }

    #[allow(dead_code)]
    pub fn open_public_sale(&mut self) {
        self.blockchain_wrapper
            .set_block_timestamp(self.public_timestamp);
    }

    #[allow(dead_code)]
    pub fn close_sale(&mut self) {
        self.blockchain_wrapper
            .set_block_timestamp(self.public_timestamp + self.sale_duration);
    }

    #[allow(dead_code)]
    pub fn set_eggs(&mut self, address: &Address, balance: u64) {
        self.blockchain_wrapper.set_nft_balance(
            address,
            &self.egg_id,
            self.egg_nonce,
            &rust_biguint!(balance),
            &{},
        );
    }

    #[allow(dead_code)]
    pub fn fill_eggs(&mut self, balance: u64) {
        self.set_eggs(&self.owner_address.clone(), balance);
        self.fill_eggs_from(&self.owner_address.clone(), balance)
            .assert_ok();
    }

    #[allow(dead_code)]
    pub fn buy(&mut self, address: &Address, egld: &num_bigint::BigUint) -> TxResult {
        return self
            .blockchain_wrapper
            .execute_tx(address, &self.contract_wrapper, egld, |sc| {
                let payment = sc.call_value().payment_as_tuple();

                sc.buy(payment.2, payment.0, payment.1);
            });
    }

    #[allow(dead_code)]
    pub fn get_eggs_balance(&mut self, address: &Address) -> num_bigint::BigUint {
        return self
            .blockchain_wrapper
            .get_esdt_balance(address, &self.egg_id, self.egg_nonce);
    }

    #[allow(dead_code)]
    pub fn get_buyed_amount(&mut self, address: &Address) -> u64 {
        let mut output = Option::None;
        self.blockchain_wrapper
            .execute_query(&self.contract_wrapper, |sc| {
                output = Some(sc.get_buyed_amount(&ManagedAddress::from_address(address)));
            })
            .assert_ok();

        assert_eq!(output.is_some(), true, "Cannot get the buyed amount");

        return output.unwrap();
    }

    #[allow(dead_code)]
    pub fn claim_balance(&mut self, caller: &Address) -> TxResult {
        return self.blockchain_wrapper.execute_tx(
            &caller,
            &self.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.claim_balance();
            },
        );
    }

    #[allow(dead_code)]
    pub fn get_all_buyers(
        &mut self,
    ) -> MultiValueEncoded<TxContextRef, MultiValue2<ManagedAddress<TxContextRef>, u64>> {
        let mut output = Option::None;
        self.blockchain_wrapper
            .execute_query(&self.contract_wrapper, |sc| {
                output = Some(sc.get_all_buyers());
            })
            .assert_ok();

        assert_eq!(output.is_some(), true, "Cannot get the buyers");

        return output.unwrap();
    }

    #[allow(dead_code)]
    pub fn fill_eggs_from(&mut self, address: &Address, balance_to_send: u64) -> TxResult {
        return self.blockchain_wrapper.execute_esdt_transfer(
            address,
            &self.contract_wrapper,
            &self.egg_id,
            self.egg_nonce,
            &rust_biguint!(balance_to_send),
            |sc| {
                let payment = sc.call_value().payment_as_tuple();

                sc.fill_egg(payment.2, payment.0, payment.1);
            },
        );
    }

    #[allow(dead_code)]
    pub fn has_access(&mut self, address: &Address) -> bool {
        let mut output = Option::None;
        self.blockchain_wrapper
            .execute_query(&self.contract_wrapper, |sc| {
                output = Some(sc.has_access(&ManagedAddress::from_address(&address)));
            })
            .assert_ok();

        return output.unwrap();
    }
}

#[allow(dead_code)]
pub fn setup_contract<ContractObjBuilder>(
    cf_builder: ContractObjBuilder,
) -> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> public_sale_mint::ContractObj<DebugApi>,
{
    DebugApi::dummy();

    let rust_zero = rust_biguint!(0u64);
    let egld_150 = rust_biguint!(150);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_zero);
    let cf_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        cf_builder,
        WASM_PATH,
    );

    let users = [
        blockchain_wrapper.create_user_account(&egld_150),
        blockchain_wrapper.create_user_account(&egld_150),
        blockchain_wrapper.create_user_account(&egld_150),
        blockchain_wrapper.create_user_account(&egld_150),
    ];

    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            sc.init(
                5,
                ManagedVec::<DebugApi, BigUint<DebugApi>>::from(vec![
                    big_uint_conv_num(10),
                    big_uint_conv_num(9),
                    big_uint_conv_num(8),
                    big_uint_conv_num(7),
                    big_uint_conv_num(6),
                ]),
                ManagedVec::<DebugApi, BigUint<DebugApi>>::from(vec![
                    big_uint_conv_num(5),
                    big_uint_conv_num(4),
                    big_uint_conv_num(3),
                    big_uint_conv_num(2),
                    big_uint_conv_num(1),
                ]),
                PUBLIC_TIMESTAMP,
                SECOND_WHITELIST_TIMESTAMP_DELTA,
                FIRST_WHITELIST_TIMESTAMP_DELTA,
                TokenIdentifier::from_esdt_bytes(&EGG_ID),
                EGG_NONCE,
                SALE_DURATION,
            );
        })
        .assert_ok();

    blockchain_wrapper.set_block_timestamp(0);
    blockchain_wrapper.add_mandos_set_account(cf_wrapper.address_ref());

    let user_first_whitelisted = blockchain_wrapper.create_user_account(&egld_150);
    let user_second_whitelisted = blockchain_wrapper.create_user_account(&egld_150);

    let mut contract_setup = ContractSetup {
        blockchain_wrapper,
        owner_address: owner_address,
        contract_wrapper: cf_wrapper,
        users,
        public_timestamp: PUBLIC_TIMESTAMP,
        first_whitelist_timestamp: PUBLIC_TIMESTAMP - FIRST_WHITELIST_TIMESTAMP_DELTA,
        second_whitelist_timestamp: PUBLIC_TIMESTAMP - SECOND_WHITELIST_TIMESTAMP_DELTA,
        egg_id: EGG_ID,
        egg_nonce: EGG_NONCE,
        sale_duration: SALE_DURATION,
        user_first_whitelisted,
        user_second_whitelisted,
    };

    contract_setup
        .add_to_first_whitelist(&contract_setup.user_first_whitelisted.clone())
        .assert_ok();
    contract_setup
        .add_to_second_whitelist(&contract_setup.user_second_whitelisted.clone())
        .assert_ok();

    return contract_setup;
}

pub fn big_uint_conv_num(value: i64) -> BigUint<DebugApi> {
    return BigUint::from_raw_handle(DebugApi::managed_type_impl().bi_new(value));
}
