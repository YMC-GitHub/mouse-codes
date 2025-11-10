//! Platform enumeration for cross-platform support

use std::fmt;

use crate::types::Button;

/// Supported operating systems/platforms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Platform {
    /// Microsoft Windows
    Windows,
    /// Linux (including X11 and Wayland)
    Linux,
    /// Apple macOS
    MacOS,
}

impl Platform {
    /// Get the current platform based on compilation target
    pub fn current() -> Self {
        #[cfg(target_os = "windows")]
        return Platform::Windows;

        #[cfg(target_os = "linux")]
        return Platform::Linux;

        #[cfg(target_os = "macos")]
        return Platform::MacOS;

        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        panic!("Unsupported platform");
    }

    /// Get code for button on this platform
    pub fn button_code(&self, button: Button) -> usize {
        button.to_code(*self)
    }

    /// Get button from code on this platform  
    pub fn button_from_code(&self, code: usize) -> Option<Button> {
        Button::from_code(code, *self)
    }

    /// Check if the given code matches the specified button on this platform
    pub fn code_matches_button(&self, code: usize, button: Button) -> bool {
        button.matches_code(code, *self)
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Platform::Windows => write!(f, "Windows"),
            Platform::Linux => write!(f, "Linux"),
            Platform::MacOS => write!(f, "macOS"),
        }
    }
}

impl std::str::FromStr for Platform {
    type Err = crate::error::MouseParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "windows" => Ok(Platform::Windows),
            "linux" => Ok(Platform::Linux),
            "macos" | "osx" => Ok(Platform::MacOS),
            _ => Err(crate::error::MouseParseError::UnknownPlatform),
        }
    }
}
