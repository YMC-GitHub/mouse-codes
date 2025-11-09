//! mouse-codes: Cross-platform mouse button code mapping and conversion
//!
//! This crate provides comprehensive mouse button definitions and cross-platform
//! code mapping for Windows, Linux, and macOS. It supports standard buttons,
//! scroll events, custom button mapping, and bidirectional conversion between
//! button names and platform-specific codes.
//!
//! # Examples
//!
//! ```rust
//! use mouse_codes::{Button, Platform, CodeMapper};
//!
//! // Parse button from string
//! let button: Button = "Left".parse().unwrap();
//! assert_eq!(button, Button::Left);
//!
//! // Convert button to platform-specific code
//! let windows_code = button.to_code(Platform::Windows);
//! let linux_code = button.to_code(Platform::Linux);
//!
//! // Parse button from code
//! let button_from_code = Button::from_code(1, Platform::Windows).unwrap();
//! assert_eq!(button_from_code, Button::Left);
//! ```
//!
//! # Features
//!
//! - `serde`: Enables serialization/deserialization support
//! - `phf`: Uses perfect hash functions for faster lookups

#![deny(missing_docs)]
#![warn(clippy::all)]

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
pub use mapping::standard::{parse_button_ignore_case};

// Re-export advanced parser functionality
pub use parser::{
    parse_button_with_aliases, parse_mouse_input,
};

use std::str::FromStr;

// Implement FromStr for Button using the standard mappings
impl FromStr for Button {
    type Err = MouseParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        mapping::standard::parse_button_from_str(s)
    }
}

/// Get the current platform based on compilation target
pub fn current_platform() -> Platform {
    #[cfg(target_os = "windows")]
    return Platform::Windows;

    #[cfg(target_os = "linux")]
    return Platform::Linux;

    #[cfg(target_os = "macos")]
    return Platform::MacOS;

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    panic!("Unsupported platform");
}