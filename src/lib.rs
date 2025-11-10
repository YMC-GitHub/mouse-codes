//! mouse-codes: Cross-platform mouse button code mapping and conversion
//!
//! This crate provides comprehensive mouse button definitions and cross-platform
//! code mapping for Windows, Linux, and macOS.

#![deny(missing_docs)]
#![warn(clippy::all)]

#[cfg(feature = "serde")]
extern crate serde;

#[cfg(feature = "phf")]
extern crate phf;

#[cfg(feature = "phf")]
extern crate lazy_static;

/// Error types for mouse parsing and mapping
pub mod error;
/// Mouse code mapping implementations
pub mod mapping;
/// Advanced mouse input parsing with alias support
pub mod parser;
/// Core type definitions for mouse buttons and platforms
pub mod types;
/// Utility functions and helpers
pub mod utils;

// Re-export main types for convenient access
pub use error::MouseParseError;
pub use mapping::custom::{CustomButton, CustomButtonMap};
pub use types::{Button, CodeMapper, MouseEvent, Platform, ScrollDirection};

// Re-export core parsing functions
pub use mapping::standard::parse_button_ignore_case;

// Re-export advanced parser functionality
pub use parser::{parse_button_with_aliases, parse_mouse_input};

// 保持向后兼容性，但标记为已弃用
#[deprecated(since = "0.1.0", note = "Use Platform::current() instead")]
/// Get the current platform based on compilation target
///
/// This function is deprecated. Use `Platform::current()` instead.
pub fn current_platform() -> Platform {
    Platform::current()
}

// 注意：移除了在 lib.rs 中的重复实现，所有方法实现都在各自的类型模块中
