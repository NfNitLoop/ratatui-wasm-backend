//! Provides a WASM-compatible backend for [Ratatui]
//! 
//! [Ratatui]: https://ratatui.rs
//! 
//! 

pub mod backend;
pub mod types;

#[cfg(feature = "parser")]
pub mod ctrl;

pub use anes;
pub use ratatui;

