// src/lib/lib.rs

// module declarations
pub mod configuration;
pub mod handlers;
pub mod service;
pub mod telemetry;

// re-exports
pub use configuration::*;
pub use service::*;
pub use telemetry::*;
