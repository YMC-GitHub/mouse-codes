//! Platform enumeration for cross-platform support

use std::fmt;

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