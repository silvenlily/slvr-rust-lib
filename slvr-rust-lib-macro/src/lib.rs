#![crate_type = "proc-macro"]

pub(crate) extern crate proc_macro;
pub(crate) use proc_macro::TokenStream;

#[cfg(feature = "tauri_wasm")]
pub(crate) mod tauri_wasm;

#[cfg(feature = "tauri_wasm")]
#[proc_macro_attribute]
pub fn tauri_wasm(input: TokenStream, item: TokenStream) -> TokenStream {
    tauri_wasm::attribute(input,item)
}



