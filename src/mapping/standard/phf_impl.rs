//! Standard mapping implementation using phf hashmaps

use crate::types::{Button, Platform};
use phf::phf_map;

static WINDOWS_CODE_MAP: phf::Map<u8, usize> = phf_map! {
    0u8 => 1,   // Button::Left
    1u8 => 2,   // Button::Right
    2u8 => 4,   // Button::Middle
    3u8 => 5,   // Button::X1
    4u8 => 6,   // Button::X2
    5u8 => 7,   // Button::Extra3
    6u8 => 8,   // Button::Extra4
    7u8 => 9,   // Button::Extra5
    8u8 => 10,  // Button::Extra6
    9u8 => 11,  // Button::Extra7
    10u8 => 12, // Button::Extra8
    #[cfg(feature = "extended")]
    11u8 => 13, // Button::Extra9
    #[cfg(feature = "extended")]
    12u8 => 14, // Button::Extra10
};

static LINUX_CODE_MAP: phf::Map<u8, usize> = phf_map! {
    0u8 => 1,   // Button::Left
    1u8 => 3,   // Button::Right
    2u8 => 2,   // Button::Middle
    3u8 => 8,   // Button::X1
    4u8 => 9,   // Button::X2
    5u8 => 10,  // Button::Extra3
    6u8 => 11,  // Button::Extra4
    7u8 => 12,  // Button::Extra5
    8u8 => 13,  // Button::Extra6
    9u8 => 14,  // Button::Extra7
    10u8 => 15, // Button::Extra8
    #[cfg(feature = "extended")]
    11u8 => 16, // Button::Extra9
    #[cfg(feature = "extended")]
    12u8 => 17, // Button::Extra10
};

static MACOS_CODE_MAP: phf::Map<u8, usize> = phf_map! {
    0u8 => 0,   // Button::Left
    1u8 => 1,   // Button::Right
    2u8 => 2,   // Button::Middle
    3u8 => 3,   // Button::X1
    4u8 => 4,   // Button::X2
    5u8 => 5,   // Button::Extra3
    6u8 => 6,   // Button::Extra4
    7u8 => 7,   // Button::Extra5
    8u8 => 8,   // Button::Extra6
    9u8 => 9,   // Button::Extra7
    10u8 => 10, // Button::Extra8
    #[cfg(feature = "extended")]
    11u8 => 11, // Button::Extra9
    #[cfg(feature = "extended")]
    12u8 => 12, // Button::Extra10
};

static WINDOWS_REVERSE_MAP: phf::Map<usize, Button> = phf_map! {
    1usize => Button::Left,
    2usize => Button::Right,
    4usize => Button::Middle,
    5usize => Button::X1,
    6usize => Button::X2,
    7usize => Button::Extra3,
    8usize => Button::Extra4,
    9usize => Button::Extra5,
    10usize => Button::Extra6,
    11usize => Button::Extra7,
    12usize => Button::Extra8,
    #[cfg(feature = "extended")]
    13usize => Button::Extra9,
    #[cfg(feature = "extended")]
    14usize => Button::Extra10,
};

static LINUX_REVERSE_MAP: phf::Map<usize, Button> = phf_map! {
    1usize => Button::Left,
    3usize => Button::Right,
    2usize => Button::Middle,
    8usize => Button::X1,
    9usize => Button::X2,
    10usize => Button::Extra3,
    11usize => Button::Extra4,
    12usize => Button::Extra5,
    13usize => Button::Extra6,
    14usize => Button::Extra7,
    15usize => Button::Extra8,
    #[cfg(feature = "extended")]
    16usize => Button::Extra9,
    #[cfg(feature = "extended")]
    17usize => Button::Extra10,
};

static MACOS_REVERSE_MAP: phf::Map<usize, Button> = phf_map! {
    0usize => Button::Left,
    1usize => Button::Right,
    2usize => Button::Middle,
    3usize => Button::X1,
    4usize => Button::X2,
    5usize => Button::Extra3,
    6usize => Button::Extra4,
    7usize => Button::Extra5,
    8usize => Button::Extra6,
    9usize => Button::Extra7,
    10usize => Button::Extra8,
    #[cfg(feature = "extended")]
    11usize => Button::Extra9,
    #[cfg(feature = "extended")]
    12usize => Button::Extra10,
};

// 为 Button 实现 u8 转换（用于 phf 键）
impl From<Button> for u8 {
    fn from(btn: Button) -> Self {
        match btn {
            Button::Left => 0,
            Button::Right => 1,
            Button::Middle => 2,
            Button::X1 => 3,
            Button::X2 => 4,
            Button::Extra3 => 5,
            Button::Extra4 => 6,
            Button::Extra5 => 7,
            Button::Extra6 => 8,
            Button::Extra7 => 9,
            Button::Extra8 => 10,
            #[cfg(feature = "extended")]
            Button::Extra9 => 11,
            #[cfg(feature = "extended")]
            Button::Extra10 => 12,
        }
    }
}

/// PHF implementation of the CodeMapper trait
pub trait CodeMapperImpl {
    /// Convert the button to a platform-specific code
    fn to_code(&self, platform: Platform) -> usize;

    /// Parse a button from a platform-specific code
    fn from_code(code: usize, platform: Platform) -> Option<Self>
    where
        Self: Sized;
}

impl CodeMapperImpl for Button {
    fn to_code(&self, platform: Platform) -> usize {
        match platform {
            Platform::Windows => WINDOWS_CODE_MAP[&u8::from(*self)],
            Platform::Linux => LINUX_CODE_MAP[&u8::from(*self)],
            Platform::MacOS => MACOS_CODE_MAP[&u8::from(*self)],
        }
    }

    fn from_code(code: usize, platform: Platform) -> Option<Self> {
        match platform {
            Platform::Windows => WINDOWS_REVERSE_MAP.get(&code).copied(),
            Platform::Linux => LINUX_REVERSE_MAP.get(&code).copied(),
            Platform::MacOS => MACOS_REVERSE_MAP.get(&code).copied(),
        }
    }
}
