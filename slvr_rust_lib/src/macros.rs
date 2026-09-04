
pub mod macro_prelude {
    #[cfg(feature = "slvr_rust_macro_encapsulate")]
    pub use slvr_rust_macro_encapsulate::encapsulate;
}

pub mod macro_lcommon {
}

pub mod macro_lib {
    #[cfg(feature = "slvr_rust_macro_encapsulate")]
    pub use slvr_rust_macro_encapsulate;
}


