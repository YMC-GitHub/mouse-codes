//! Custom mouse button mapping support

use std::collections::HashMap;

use crate::{
    error::MouseParseError,
    types::{Button, CodeMapper, Platform},
};

/// A custom mouse button that extends the standard button set
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CustomButton {
    /// A standard mouse button
    Standard(Button),
    /// A custom-defined mouse button with a name
    Custom(&'static str),
}

impl fmt::Display for CustomButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomButton::Standard(btn) => write!(f, "{}", btn),
            CustomButton::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// A mapping for custom mouse buttons with platform-specific codes
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CustomButtonMap {
    name: String,
    mappings: HashMap<CustomButton, [Option<usize>; 3]>, // [Windows, Linux, macOS]
    reverse_mappings: [HashMap<usize, CustomButton>; 3], // [Windows, Linux, macOS]
}

impl CustomButtonMap {
    /// Create a new empty custom button map
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            mappings: HashMap::new(),
            reverse_mappings: [
                HashMap::new(),
                HashMap::new(),
                HashMap::new(),
            ],
        }
    }

    /// Add a custom button with platform-specific codes
    pub fn add_button(
        &mut self,
        button: CustomButton,
        windows_code: Option<usize>,
        linux_code: Option<usize>,
        macos_code: Option<usize>,
    ) -> Result<(), MouseParseError> {
        if self.mappings.contains_key(&button) {
            return Err(MouseParseError::DuplicateCustomButton(button.to_string()));
        }

        // Store forward mapping
        self.mappings.insert(button, [windows_code, linux_code, macos_code]);

        // Store reverse mappings for each platform
        if let Some(code) = windows_code {
            self.reverse_mappings[0].insert(code, button);
        }
        if let Some(code) = linux_code {
            self.reverse_mappings[1].insert(code, button);
        }
        if let Some(code) = macos_code {
            self.reverse_mappings[2].insert(code, button);
        }

        Ok(())
    }

    /// Get the name of this custom map
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl CodeMapper for CustomButton {
    fn to_code(&self, platform: Platform) -> usize {
        match self {
            CustomButton::Standard(btn) => btn.to_code(platform),
            CustomButton::Custom(_) => panic!("Custom buttons require a CustomButtonMap for conversion"),
        }
    }

    fn from_code(code: usize, platform: Platform) -> Option<Self> {
        Button::from_code(code, platform).map(CustomButton::Standard)
    }
}

impl CodeMapper for CustomButtonMap {
    fn to_code(&self, platform: Platform) -> usize {
        panic!("Use CustomButtonMap::get_code_for_button instead");
    }

    fn from_code(&self, code: usize, platform: Platform) -> Option<CustomButton> {
        let idx = match platform {
            Platform::Windows => 0,
            Platform::Linux => 1,
            Platform::MacOS => 2,
        };

        // Check custom mappings first, then fall back to standard buttons
        self.reverse_mappings[idx].get(&code).copied()
            .or_else(|| Button::from_code(code, platform).map(CustomButton::Standard))
    }
}

impl CustomButtonMap {
    /// Get the platform-specific code for a custom button
    pub fn get_code_for_button(&self, button: &CustomButton, platform: Platform) -> Option<usize> {
        match button {
            CustomButton::Standard(btn) => Some(btn.to_code(platform)),
            CustomButton::Custom(_) => {
                let idx = match platform {
                    Platform::Windows => 0,
                    Platform::Linux => 1,
                    Platform::MacOS => 2,
                };
                self.mappings.get(button).and_then(|codes| codes[idx])
            }
        }
    }
}