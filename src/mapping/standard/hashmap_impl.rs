//! Standard mapping implementation using std::collections::HashMap

use crate::types::{Button, Platform};
use std::collections::HashMap;
use std::sync::LazyLock;

static WINDOWS_CODE_MAP: LazyLock<HashMap<Button, usize>> = LazyLock::new(|| {
    HashMap::from([
        (Button::Left, 1),
        (Button::Right, 2),
        (Button::Middle, 4),
        (Button::X1, 5),
        (Button::X2, 6),
        (Button::Extra3, 7),
        (Button::Extra4, 8),
        (Button::Extra5, 9),
        (Button::Extra6, 10),
        (Button::Extra7, 11),
        (Button::Extra8, 12),
        #[cfg(feature = "extended")]
        (Button::Extra9, 13),
        #[cfg(feature = "extended")]
        (Button::Extra10, 14),
    ])
});

static LINUX_CODE_MAP: LazyLock<HashMap<Button, usize>> = LazyLock::new(|| {
    HashMap::from([
        (Button::Left, 1),
        (Button::Right, 3),
        (Button::Middle, 2),
        (Button::X1, 8),
        (Button::X2, 9),
        (Button::Extra3, 10),
        (Button::Extra4, 11),
        (Button::Extra5, 12),
        (Button::Extra6, 13),
        (Button::Extra7, 14),
        (Button::Extra8, 15),
        #[cfg(feature = "extended")]
        (Button::Extra9, 16),
        #[cfg(feature = "extended")]
        (Button::Extra10, 17),
    ])
});

static MACOS_CODE_MAP: LazyLock<HashMap<Button, usize>> = LazyLock::new(|| {
    HashMap::from([
        (Button::Left, 0),
        (Button::Right, 1),
        (Button::Middle, 2),
        (Button::X1, 3),
        (Button::X2, 4),
        (Button::Extra3, 5),
        (Button::Extra4, 6),
        (Button::Extra5, 7),
        (Button::Extra6, 8),
        (Button::Extra7, 9),
        (Button::Extra8, 10),
        #[cfg(feature = "extended")]
        (Button::Extra9, 11),
        #[cfg(feature = "extended")]
        (Button::Extra10, 12),
    ])
});

static WINDOWS_REVERSE_MAP: LazyLock<HashMap<usize, Button>> = LazyLock::new(|| {
    HashMap::from([
        (1, Button::Left),
        (2, Button::Right),
        (4, Button::Middle),
        (5, Button::X1),
        (6, Button::X2),
        (7, Button::Extra3),
        (8, Button::Extra4),
        (9, Button::Extra5),
        (10, Button::Extra6),
        (11, Button::Extra7),
        (12, Button::Extra8),
        #[cfg(feature = "extended")]
        (13, Button::Extra9),
        #[cfg(feature = "extended")]
        (14, Button::Extra10),
    ])
});

static LINUX_REVERSE_MAP: LazyLock<HashMap<usize, Button>> = LazyLock::new(|| {
    HashMap::from([
        (1, Button::Left),
        (3, Button::Right),
        (2, Button::Middle),
        (8, Button::X1),
        (9, Button::X2),
        (10, Button::Extra3),
        (11, Button::Extra4),
        (12, Button::Extra5),
        (13, Button::Extra6),
        (14, Button::Extra7),
        (15, Button::Extra8),
        #[cfg(feature = "extended")]
        (16, Button::Extra9),
        #[cfg(feature = "extended")]
        (17, Button::Extra10),
    ])
});

static MACOS_REVERSE_MAP: LazyLock<HashMap<usize, Button>> = LazyLock::new(|| {
    HashMap::from([
        (0, Button::Left),
        (1, Button::Right),
        (2, Button::Middle),
        (3, Button::X1),
        (4, Button::X2),
        (5, Button::Extra3),
        (6, Button::Extra4),
        (7, Button::Extra5),
        (8, Button::Extra6),
        (9, Button::Extra7),
        (10, Button::Extra8),
        #[cfg(feature = "extended")]
        (11, Button::Extra9),
        #[cfg(feature = "extended")]
        (12, Button::Extra10),
    ])
});

/// HashMap implementation of the CodeMapper trait
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
            Platform::Windows => *WINDOWS_CODE_MAP
                .get(self)
                .expect("Invalid button for Windows platform"),
            Platform::Linux => *LINUX_CODE_MAP
                .get(self)
                .expect("Invalid button for Linux platform"),
            Platform::MacOS => *MACOS_CODE_MAP
                .get(self)
                .expect("Invalid button for macOS platform"),
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
