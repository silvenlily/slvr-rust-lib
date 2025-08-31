#[cfg(feature = "tauri_wasm")]
pub mod tauri_wasm;

#[cfg(feature = "encapsulation")]
pub mod encapsulation;

pub mod deps {
    #[cfg(feature = "serde")]
    pub use serde;
}
