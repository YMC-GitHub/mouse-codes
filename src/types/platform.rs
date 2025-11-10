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
