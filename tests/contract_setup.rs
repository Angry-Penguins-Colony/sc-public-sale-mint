use elrond_wasm::types::{Address, ManagedAddress, ManagedVec, TokenIdentifier};
use elrond_wasm_debug::{rust_biguint, testing_framework::*, tx_mock::TxResult, DebugApi};
use public_sale_mint::{whitelist::WhitelistModule, *};

pub const WASM_PATH: &'static str = "output/empty.wasm";

pub struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> public_sale_mint::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub users: [Address; 4],
    pub contract_wrapper:
        ContractObjWrapper<public_sale_mint::ContractObj<DebugApi>, ContractObjBuilder>,
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
                sc.add_to_first_whitelist(ManagedAddress::from_address(&address));
            },
        );

        return tx_result;
    }

    #[allow(dead_code)]
    pub fn is_first_whitelisted(&mut self, address: Address) -> bool {
        let mut output = Option::None;
        self.blockchain_wrapper
            .execute_query(&self.contract_wrapper, |sc| {
                output = Some(sc.check_contains_first(ManagedAddress::from_address(&address)));
            })
            .assert_ok();

        return output.unwrap();
    }

    #[allow(dead_code)]
    pub fn add_to_second_whitelist(&mut self, address: Address) -> TxResult {
        let tx_result = self.blockchain_wrapper.execute_tx(
            &self.owner_address,
            &self.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.add_to_second_whitelist(ManagedAddress::from_address(&address));
            },
        );

        return tx_result;
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
                0,
                0,
                0,
                TokenIdentifier::from_esdt_bytes(b"TOKEN"),
                3,
            );
        })
        .assert_ok();

    blockchain_wrapper.add_mandos_set_account(cf_wrapper.address_ref());

    ContractSetup {
        blockchain_wrapper,
        owner_address: owner_address,
        contract_wrapper: cf_wrapper,
        users,
    }
}
