elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub const ERR_NOT_OWNER: &str = "Endpoint can only be called by owner";

#[elrond_wasm::module]
pub trait WhitelistModule {
    #[view(timestamp_public_sale)]
    #[storage_mapper("timestamp_public_sale")]
    fn timestamp_public_sale(&self) -> SingleValueMapper<u64>;

    #[view(timestamp_second_whitelist)]
    #[storage_mapper("timestamp_second_whitelist")]
    fn timestamp_second_whitelist(&self) -> SingleValueMapper<u64>;

    #[view(timestamp_first_whitelist)]
    #[storage_mapper("timestamp_first_whitelist")]
    fn timestamp_first_whitelist(&self) -> SingleValueMapper<u64>;

    #[endpoint]
    fn has_access(&self, address: &ManagedAddress) -> bool {
        let now = self.blockchain().get_block_timestamp();

        let public_sale_timestamp = self.timestamp_public_sale().get();

        if now >= public_sale_timestamp {
            return true;
        } else if now >= self.timestamp_second_whitelist().get()
            && self.check_contains_second(&address)
        {
            return true;
        } else if now >= self.timestamp_first_whitelist().get()
            && self.check_contains_first(&address)
        {
            return true;
        }

        return false;
    }

    // ===
    // FIRST WHITELIST
    #[endpoint]
    #[only_owner]
    fn add_to_first_whitelist(&self, item: &ManagedAddress) {
        self.blockchain().check_caller_is_owner();
        self.first_whitelist_mapper().add(item);
    }

    #[endpoint]
    #[only_owner]
    fn remove_from_first_whitelist(&self, item: &ManagedAddress) {
        self.blockchain().check_caller_is_owner();
        self.first_whitelist_mapper().remove(item);
    }

    #[endpoint]
    fn check_contains_first(&self, item: &ManagedAddress) -> bool {
        self.first_whitelist_mapper().contains(item)
    }

    #[endpoint]
    fn require_contains_first(&self, item: &ManagedAddress) {
        self.first_whitelist_mapper().require_whitelisted(item);
    }

    #[storage_mapper("first_whitelist_mapper")]
    fn first_whitelist_mapper(&self) -> WhitelistMapper<Self::Api, ManagedAddress>;

    // ===
    // SECOND WHITELIST
    #[endpoint]
    #[only_owner]
    fn add_to_second_whitelist(&self, item: &ManagedAddress) {
        self.blockchain().check_caller_is_owner();
        self.second_whitelist_mapper().add(item);
    }

    #[endpoint]
    #[only_owner]
    fn remove_from_second_whitelist(&self, item: &ManagedAddress) {
        self.blockchain().check_caller_is_owner();
        self.second_whitelist_mapper().remove(item);
    }

    #[endpoint]
    fn check_contains_second(&self, item: &ManagedAddress) -> bool {
        self.second_whitelist_mapper().contains(item)
    }

    #[endpoint]
    fn require_contains_second(&self, item: &ManagedAddress) {
        self.second_whitelist_mapper().require_whitelisted(item);
    }

    #[storage_mapper("second_whitelist_mapper")]
    fn second_whitelist_mapper(&self) -> WhitelistMapper<Self::Api, ManagedAddress>;
}
