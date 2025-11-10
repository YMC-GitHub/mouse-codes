use std::fmt;

use super::Button;

/// Mouse event type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MouseEvent {
    /// Button press event
    Press(Button),
    /// Button release event
    Release(Button),
    /// Scroll event with direction and amount
    Scroll(ScrollDirection, i32),
    /// Mouse movement event (x, y coordinates)
    Move(i32, i32),
    /// Mouse movement relative to previous position (dx, dy)
    RelativeMove(i32, i32),
}

/// Scroll direction enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScrollDirection {
    /// Vertical scroll up
    VerticalUp,
    /// Vertical scroll down
    VerticalDown,
    /// Horizontal scroll left
    HorizontalLeft,
    /// Horizontal scroll right
    HorizontalRight,
}

impl fmt::Display for ScrollDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScrollDirection::VerticalUp => write!(f, "VerticalUp"),
            ScrollDirection::VerticalDown => write!(f, "VerticalDown"),
            ScrollDirection::HorizontalLeft => write!(f, "HorizontalLeft"),
            ScrollDirection::HorizontalRight => write!(f, "HorizontalRight"),
        }
    }
}

impl fmt::Display for MouseEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MouseEvent::Press(button) => write!(f, "Press({})", button),
            MouseEvent::Release(button) => write!(f, "Release({})", button),
            MouseEvent::Scroll(dir, amount) => write!(f, "Scroll({}, {})", dir, amount),
            MouseEvent::Move(x, y) => write!(f, "Move({}, {})", x, y),
            MouseEvent::RelativeMove(dx, dy) => write!(f, "RelativeMove({}, {})", dx, dy),
        }
    }
}
