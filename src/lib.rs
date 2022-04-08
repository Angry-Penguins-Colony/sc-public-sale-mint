#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::derive::contract]
pub trait PublicSaleMint {
    #[init]
    fn init(&self) {}
}
