#![crate_type = "proc-macro"]

pub(crate) extern crate proc_macro;
pub(crate) use proc_macro::TokenStream;

#[cfg(feature = "tauri_wasm")]
mod tauri_wasm;

#[cfg(feature = "tauri_wasm")]
#[proc_macro_attribute]
pub fn tauri_command(attr: TokenStream, item: TokenStream) -> TokenStream {
    tauri_wasm::command_attribute(attr, item)
}

#[cfg(feature = "tauri_wasm")]
#[proc_macro_attribute]
pub fn tauri_command_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    tauri_wasm::command_impl_attribute(attr, item)
}

#[cfg(feature = "tauri_wasm")]
#[proc_macro_attribute]
pub fn tauri_response(attr: TokenStream, item: TokenStream) -> TokenStream {
    tauri_wasm::response_attribute(attr, item)
}

#[cfg(feature = "encapsulation")]
mod encapsulation;

#[cfg(feature = "encapsulation")]
#[proc_macro_attribute]
pub fn encapsulate(attr: TokenStream,  item: TokenStream) -> TokenStream {
    encapsulation::encapsulate(attr, item)
}

