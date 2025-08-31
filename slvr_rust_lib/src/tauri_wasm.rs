
pub mod prelude {
    pub use slvr_rust_lib_macro::tauri_command;
    pub use slvr_rust_lib_macro::tauri_response;
    pub use slvr_rust_lib_macro::tauri_command_impl;
    pub use slvr_rust_lib_common::tauri_wasm::*;
}

pub mod build {
    pub use slvr_rust_lib_build::tauri_wasm::resolve_message_structs;
}

pub use prelude::*;

