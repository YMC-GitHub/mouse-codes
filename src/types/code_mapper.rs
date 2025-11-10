use super::Platform;

/// Trait for types that can be converted to and from platform-specific mouse codes
pub trait CodeMapper {
    /// Convert the button to a platform-specific code
    fn to_code(&self, platform: Platform) -> usize;

    /// Parse a button from a platform-specific code
    fn from_code(code: usize, platform: Platform) -> Option<Self>
    where
        Self: Sized;
}
