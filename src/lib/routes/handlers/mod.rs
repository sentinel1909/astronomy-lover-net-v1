// src/lib/routes/handlers/mod.rs

// module declarations
pub mod count;
pub mod echo;
pub mod fetch;
pub mod health;
pub mod metrics;
pub mod ping;
pub mod static_files;

// re-exports
pub use count::*;
pub use echo::*;
pub use fetch::*;
pub use health::*;
pub use metrics::*;
pub use ping::*;
pub use static_files::*;
