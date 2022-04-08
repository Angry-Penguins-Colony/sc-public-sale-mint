elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub const ERR_NOT_OWNER: &str = "Endpoint can only be called by owner";

#[elrond_wasm::module]
pub trait WhitelistModule {
    #[endpoint]
    fn has_access(&self, address: ManagedAddress) -> bool {
        panic!("Not implemented yet")
    }

    // ===
    // FIRST WHITELIST
    #[endpoint]
    fn add_to_first_whitelist(&self, item: ManagedAddress) {
        self.blockchain().check_caller_is_owner();
        self.first_whitelist_mapper().add(&item);
    }

    #[endpoint]
    fn remove_from_first_whitelist(&self, item: ManagedAddress) {
        self.blockchain().check_caller_is_owner();
        self.first_whitelist_mapper().remove(&item);
    }

    #[endpoint]
    fn check_contains_first(&self, item: ManagedAddress) -> bool {
        self.first_whitelist_mapper().contains(&item)
    }

    #[endpoint]
    fn require_contains_first(&self, item: ManagedAddress) {
        self.first_whitelist_mapper().require_whitelisted(&item);
    }

    #[storage_mapper("whitelistMapper")]
    fn first_whitelist_mapper(&self) -> WhitelistMapper<Self::Api, ManagedAddress>;

    // ===
    // SECOND WHITELIST
    #[endpoint]
    fn add_to_second_whitelist(&self, item: ManagedAddress) {
        self.blockchain().check_caller_is_owner();
        self.second_whitelist_mapper().add(&item);
    }

    #[endpoint]
    fn remove_from_second_whitelist(&self, item: ManagedAddress) {
        self.blockchain().check_caller_is_owner();
        self.second_whitelist_mapper().remove(&item);
    }

    #[endpoint]
    fn check_contains_second(&self, item: ManagedAddress) -> bool {
        self.second_whitelist_mapper().contains(&item)
    }

    #[endpoint]
    fn require_contains_second(&self, item: ManagedAddress) {
        self.second_whitelist_mapper().require_whitelisted(&item);
    }

    #[storage_mapper("whitelistMapper")]
    fn second_whitelist_mapper(&self) -> WhitelistMapper<Self::Api, ManagedAddress>;
}
