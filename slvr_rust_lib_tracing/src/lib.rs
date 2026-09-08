pub mod log;
pub mod tracing_struct_ext;

pub mod lcommon {
    pub mod log {
        pub use crate::log::LoggingConfig;
        pub use crate::log::Level;
    }
}

pub mod prelude {
    pub use crate::tracing_struct_ext::TracingStructExt;
    pub use tracing::trace;
    pub use tracing::debug;
    pub use tracing::info;
    pub use tracing::warn;
    pub use tracing::error;
}

pub mod lib {
    pub use tracing;
    pub use tracing_log;
    pub use tracing_subscriber;
}