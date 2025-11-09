//! Utility functions for mouse input handling

use crate::types::{Button, Platform};

/// Get a list of all standard mouse buttons
pub fn all_standard_buttons() -> Vec<Button> {
    vec![
        Button::Left,
        Button::Right,
        Button::Middle,
        Button::X1,
        Button::X2,
        Button::Extra3,
        Button::Extra4,
        Button::Extra5,
        Button::Extra6,
        Button::Extra7,
        Button::Extra8,
    ]
}

/// Convert a platform-specific button code to a human-readable name
pub fn code_to_name(code: usize, platform: Platform) -> Option<String> {
    Button::from_code(code, platform).map(|btn| btn.to_string())
}

/// Get a mapping of all buttons to their codes for a specific platform
pub fn platform_button_mapping(platform: Platform) -> std::collections::HashMap<Button, usize> {
    all_standard_buttons()
        .into_iter()
        .map(|btn| (btn, btn.to_code(platform)))
        .collect()
}

/// Get a reverse mapping of codes to buttons for a specific platform
pub fn platform_code_mapping(platform: Platform) -> std::collections::HashMap<usize, Button> {
    all_standard_buttons()
        .into_iter()
        .map(|btn| (btn.to_code(platform), btn))
        .collect()
}