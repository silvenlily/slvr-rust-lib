#![allow(unused_imports)]

pub mod macros;

pub mod prelude {
    pub use crate::lcommon;
    pub use crate::macros::macro_prelude::*;

    #[cfg(feature = "slvr_rust_lib_common")]
    pub use slvr_rust_lib_common::prelude::*;

    #[cfg(feature = "slvr_rust_lib_sync")]
    pub use slvr_rust_lib_sync::prelude::*;

    #[cfg(feature = "slvr_rust_lib_tracing")]
    pub use slvr_rust_lib_tracing::prelude::*;
}

pub mod lcommon {
    pub use crate::macros::macro_lcommon::*;

    #[cfg(feature = "slvr_rust_lib_common")]
    pub use slvr_rust_lib_common::lcommon::*;

    #[cfg(feature = "slvr_rust_lib_sync")]
    pub use slvr_rust_lib_sync::lcommon::*;

    #[cfg(feature = "slvr_rust_lib_tracing")]
    pub use slvr_rust_lib_tracing::lcommon::*;
}

pub mod lib {
    pub use crate::macros::macro_lib::*;

    #[cfg(feature = "slvr_rust_lib_common")]
    pub use slvr_rust_lib_common;

    #[cfg(feature = "slvr_rust_lib_sync")]
    pub use slvr_rust_lib_sync;

    #[cfg(feature = "slvr_rust_lib_tracing")]
    pub use slvr_rust_lib_tracing;
}