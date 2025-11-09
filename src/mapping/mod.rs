//! Mouse button code mapping implementations
//!
//! This module provides standard and custom mapping functionality for
//! converting between mouse buttons and platform-specific codes.

pub mod custom;
pub mod standard;

/// Re-export key types and traits from submodules
pub use custom::{CustomButton, CustomButtonMap};
pub use standard::parse_button_from_str;

/// Helper function to get platform-specific code for a button
///
/// Uses the standard mapping by default
pub fn get_code(button: &crate::types::Button, platform: crate::types::Platform) -> usize {
    standard::CodeMapper::to_code(button, platform)
}

/// Helper function to get button from platform-specific code
///
/// Uses the standard mapping by default
pub fn get_button(code: usize, platform: crate::types::Platform) -> Option<crate::types::Button> {
    standard::CodeMapper::from_code(code, platform)
}

/// Get all standard button mappings for a platform as a hashmap
pub fn standard_mapping(platform: crate::types::Platform) -> std::collections::HashMap<crate::types::Button, usize> {
    crate::utils::platform_button_mapping(platform)
}

/// Get reverse standard mapping (code to button) for a platform
pub fn reverse_standard_mapping(platform: crate::types::Platform) -> std::collections::HashMap<usize, crate::types::Button> {
    crate::utils::platform_code_mapping(platform)
}