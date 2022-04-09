use elrond_wasm::{
    contract_base::ContractBase,
    types::{Address, ManagedAddress, ManagedVec, TokenIdentifier},
};
use elrond_wasm_debug::{rust_biguint, testing_framework::*, tx_mock::TxResult, DebugApi};
use public_sale_mint::{whitelist::WhitelistModule, *};

pub const WASM_PATH: &'static str = "output/empty.wasm";
pub const PUBLIC_TIMESTAMP: u64 = 120;
pub const SECOND_WHITELIST_TIMESTAMP_DELTA: u64 = 20;
pub const FIRST_WHITELIST_TIMESTAMP_DELTA: u64 = 40;
pub const EGG_ID: [u8; 3] = *b"EGG";
pub const EGG_NONCE: u64 = 1;

pub struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> public_sale_mint::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub users: [Address; 4],
    pub contract_wrapper:
        ContractObjWrapper<public_sale_mint::ContractObj<DebugApi>, ContractObjBuilder>,
    pub public_timestamp: u64,
    pub first_whitelist_timestamp: u64,
    pub second_whitelist_timestamp: u64,
    pub egg_id: [u8; 3],
    pub egg_nonce: u64,
}

impl<ContractObjBuilder> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> public_sale_mint::ContractObj<DebugApi>,
{
    #[allow(dead_code)]
    pub fn add_to_first_whitelist(&mut self, address: Address) -> TxResult {
        let tx_result = self.blockchain_wrapper.execute_tx(
            &self.owner_address,
            &self.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.add_to_first_whitelist(&ManagedAddress::from_address(&address));
            },
        );

        return tx_result;
    }

    #[allow(dead_code)]
    pub fn add_to_second_whitelist(&mut self, address: Address) -> TxResult {
        let tx_result = self.blockchain_wrapper.execute_tx(
            &self.owner_address,
            &self.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.add_to_second_whitelist(&ManagedAddress::from_address(&address));
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
    pub fn set_eggs(&mut self, address: &Address, balance: u64) {
        self.blockchain_wrapper.set_nft_balance(
            address,
            &self.egg_id,
            self.egg_nonce,
            &rust_biguint!(balance),
            &{},
        )
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
                output = Some(sc.has_access(ManagedAddress::from_address(&address)));
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
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_zero);
    let cf_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        cf_builder,
        WASM_PATH,
    );

    let users = [
        blockchain_wrapper.create_user_account(&rust_zero),
        blockchain_wrapper.create_user_account(&rust_zero),
        blockchain_wrapper.create_user_account(&rust_zero),
        blockchain_wrapper.create_user_account(&rust_zero),
    ];

    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            sc.init(
                5,
                ManagedVec::from(vec![1u64, 2u64, 3u64, 4u64, 5u64]),
                ManagedVec::from(vec![1u64, 2u64, 3u64, 4u64, 5u64]),
                PUBLIC_TIMESTAMP,
                SECOND_WHITELIST_TIMESTAMP_DELTA,
                FIRST_WHITELIST_TIMESTAMP_DELTA,
                TokenIdentifier::from_esdt_bytes(&EGG_ID),
                EGG_NONCE,
            );
        })
        .assert_ok();

    blockchain_wrapper.set_block_timestamp(0);
    blockchain_wrapper.add_mandos_set_account(cf_wrapper.address_ref());

    ContractSetup {
        blockchain_wrapper,
        owner_address: owner_address,
        contract_wrapper: cf_wrapper,
        users,
        public_timestamp: PUBLIC_TIMESTAMP,
        first_whitelist_timestamp: PUBLIC_TIMESTAMP - FIRST_WHITELIST_TIMESTAMP_DELTA,
        second_whitelist_timestamp: PUBLIC_TIMESTAMP - SECOND_WHITELIST_TIMESTAMP_DELTA,
        egg_id: EGG_ID,
        egg_nonce: EGG_NONCE,
    }
}
