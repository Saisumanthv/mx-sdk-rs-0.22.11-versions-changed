dharitri_wasm::imports!();

/// Example of a module that lies in the same crate.
#[dharitri_wasm::module]
pub trait InternalModuleB {
    #[view]
    fn call_mod_b(&self) {}
}
