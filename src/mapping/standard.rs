use crate::types::{Button, Platform};

// Standard mouse button mappings for each platform
// (name, button, windows_code, linux_code, macos_code)
const STANDARD_BUTTON_MAPPINGS: &[(&str, Button, usize, usize, usize)] = &[
    ("Left", Button::Left, 1, 1, 0),
    ("Right", Button::Right, 2, 3, 1),
    ("Middle", Button::Middle, 4, 2, 2),
    ("X1", Button::X1, 5, 8, 3),
    ("X2", Button::X2, 6, 9, 4),
    ("Extra3", Button::Extra3, 7, 10, 5),
    ("Extra4", Button::Extra4, 8, 11, 6),
    ("Extra5", Button::Extra5, 9, 12, 7),
    ("Extra6", Button::Extra6, 10, 13, 8),
    ("Extra7", Button::Extra7, 11, 14, 9),
    ("Extra8", Button::Extra8, 12, 15, 10),
];

/// Parse a button from a string with exact matching
pub fn parse_button_from_str(s: &str) -> Result<Button, crate::error::MouseParseError> {
    STANDARD_BUTTON_MAPPINGS
        .iter()
        .find(|(name, _, _, _, _)| *name == s)
        .map(|(_, button, _, _, _)| *button)
        .ok_or_else(|| crate::error::MouseParseError::UnknownButton(s.to_string()))
}

/// Parse a button from a string with case-insensitive matching
pub fn parse_button_ignore_case(s: &str) -> Result<Button, crate::error::MouseParseError> {
    STANDARD_BUTTON_MAPPINGS
        .iter()
        .find(|(name, _, _, _, _)| name.eq_ignore_ascii_case(s))
        .map(|(_, button, _, _, _)| *button)
        .ok_or_else(|| crate::error::MouseParseError::UnknownButton(s.to_string()))
}

impl crate::types::CodeMapper for Button {
    fn to_code(&self, platform: Platform) -> usize {
        STANDARD_BUTTON_MAPPINGS
            .iter()
            .find(|(_, btn, _, _, _)| btn == self)
            .map(|(_, _, win, linux, mac)| match platform {
                Platform::Windows => *win,
                Platform::Linux => *linux,
                Platform::MacOS => *mac,
            })
            .unwrap_or_else(|| panic!("Unsupported button in standard mapping: {:?}", self))
    }

    fn from_code(code: usize, platform: Platform) -> Option<Self> {
        STANDARD_BUTTON_MAPPINGS
            .iter()
            .find(|(_, _, win, linux, mac)| match platform {
                Platform::Windows => *win == code,
                Platform::Linux => *linux == code,
                Platform::MacOS => *mac == code,
            })
            .map(|(_, button, _, _, _)| *button)
    }
}