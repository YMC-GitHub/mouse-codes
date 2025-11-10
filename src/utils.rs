//! Utility functions for mouse input handling

use crate::types::{Button, CodeMapper, Platform};
use std::collections::HashMap;

/// Get a list of all standard mouse buttons
pub fn all_standard_buttons() -> Vec<Button> {
    #[cfg(not(feature = "extended"))]
    let buttons = vec![
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
    ];

    #[cfg(feature = "extended")]
    let mut buttons = vec![
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
    ];

    #[cfg(feature = "extended")]
    {
        buttons.push(Button::Extra9);
        buttons.push(Button::Extra10);
    }

    buttons
}

/// Convert a platform-specific button code to a human-readable name
pub fn code_to_name(code: usize, platform: Platform) -> Option<String> {
    <Button as CodeMapper>::from_code(code, platform).map(|btn| btn.to_string())
}

/// Get a mapping of all buttons to their codes for a specific platform
pub fn platform_button_mapping(platform: Platform) -> HashMap<Button, usize> {
    all_standard_buttons()
        .into_iter()
        .map(|btn| (btn, <Button as CodeMapper>::to_code(&btn, platform)))
        .collect()
}

/// Get a reverse mapping of codes to buttons for a specific platform
pub fn platform_code_mapping(platform: Platform) -> HashMap<usize, Button> {
    all_standard_buttons()
        .into_iter()
        .map(|btn| (<Button as CodeMapper>::to_code(&btn, platform), btn))
        .collect()
}
