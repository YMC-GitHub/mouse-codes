use std::fmt;

#[cfg(feature = "phf")]
use phf::PhfHash;
#[cfg(feature = "phf")]
use phf_shared::PhfBorrow; // 从 phf_shared 导入 PhfBorrow

/// Mouse button enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Button {
    /// Left mouse button
    Left,
    /// Right mouse button
    Right,
    /// Middle mouse button (scroll wheel press)
    Middle,
    /// X1 button (usually back)
    X1,
    /// X2 button (usually forward)
    X2,
    /// Extra button 3
    Extra3,
    /// Extra button 4
    Extra4,
    /// Extra button 5
    Extra5,
    /// Extra button 6
    Extra6,
    /// Extra button 7
    Extra7,
    /// Extra button 8
    Extra8,
    #[cfg(feature = "extended")]
    /// Extra button 9 (extended feature)
    Extra9,
    #[cfg(feature = "extended")]
    /// Extra button 10 (extended feature)
    Extra10,
}

#[cfg(feature = "phf")]
impl PhfHash for Button {
    fn phf_hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().phf_hash(state);
    }
}

#[cfg(feature = "phf")]
impl PhfBorrow<Button> for Button {
    fn borrow(&self) -> &Button {
        self
    }
}

// 为 phf 哈希表查找实现 PhfBorrow<Button> for &Button
#[cfg(feature = "phf")]
impl PhfBorrow<Button> for &Button {
    fn borrow(&self) -> &Button {
        self
    }
}

impl Button {
    /// Get the string representation of the button
    pub fn as_str(&self) -> &'static str {
        match self {
            Button::Left => "Left",
            Button::Right => "Right",
            Button::Middle => "Middle",
            Button::X1 => "X1",
            Button::X2 => "X2",
            Button::Extra3 => "Extra3",
            Button::Extra4 => "Extra4",
            Button::Extra5 => "Extra5",
            Button::Extra6 => "Extra6",
            Button::Extra7 => "Extra7",
            Button::Extra8 => "Extra8",
            #[cfg(feature = "extended")]
            Button::Extra9 => "Extra9",
            #[cfg(feature = "extended")]
            Button::Extra10 => "Extra10",
        }
    }
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
