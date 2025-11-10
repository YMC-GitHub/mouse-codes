//! Standard mouse button code mappings for cross-platform compatibility

use crate::types::{Button, Platform};

/// Parse a button from a string with exact matching
pub fn parse_button_from_str(s: &str) -> Result<Button, crate::error::MouseParseError> {
    match s {
        "Left" => Ok(Button::Left),
        "Right" => Ok(Button::Right),
        "Middle" => Ok(Button::Middle),
        "X1" => Ok(Button::X1),
        "X2" => Ok(Button::X2),
        "Extra3" => Ok(Button::Extra3),
        "Extra4" => Ok(Button::Extra4),
        "Extra5" => Ok(Button::Extra5),
        "Extra6" => Ok(Button::Extra6),
        "Extra7" => Ok(Button::Extra7),
        "Extra8" => Ok(Button::Extra8),
        #[cfg(feature = "extended")]
        "Extra9" => Ok(Button::Extra9),
        #[cfg(feature = "extended")]
        "Extra10" => Ok(Button::Extra10),
        _ => Err(crate::error::MouseParseError::UnknownButton(s.to_string())),
    }
}

/// Parse a button from a string with case-insensitive matching
pub fn parse_button_ignore_case(s: &str) -> Result<Button, crate::error::MouseParseError> {
    let s = s.to_ascii_lowercase();
    match s.as_str() {
        "left" => Ok(Button::Left),
        "right" => Ok(Button::Right),
        "middle" => Ok(Button::Middle),
        "x1" => Ok(Button::X1),
        "x2" => Ok(Button::X2),
        "extra3" => Ok(Button::Extra3),
        "extra4" => Ok(Button::Extra4),
        "extra5" => Ok(Button::Extra5),
        "extra6" => Ok(Button::Extra6),
        "extra7" => Ok(Button::Extra7),
        "extra8" => Ok(Button::Extra8),
        #[cfg(feature = "extended")]
        "extra9" => Ok(Button::Extra9),
        #[cfg(feature = "extended")]
        "extra10" => Ok(Button::Extra10),
        _ => Err(crate::error::MouseParseError::UnknownButton(s)),
    }
}

// 根据特性选择不同的实现
#[cfg(feature = "phf")]
mod phf_impl;
#[cfg(feature = "phf")]
pub use phf_impl::CodeMapperImpl as CodeMapper;

#[cfg(not(feature = "phf"))]
mod hashmap_impl;
#[cfg(not(feature = "phf"))]
pub use hashmap_impl::CodeMapperImpl as CodeMapper;

// 为 Button 实现 CodeMapper  trait（转发到具体实现）
impl crate::types::CodeMapper for Button {
    fn to_code(&self, platform: Platform) -> usize {
        <Self as CodeMapper>::to_code(self, platform)
    }

    fn from_code(code: usize, platform: Platform) -> Option<Self> {
        <Self as CodeMapper>::from_code(code, platform)
    }
}
