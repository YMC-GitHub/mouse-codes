//! Parsing utilities for mouse buttons and events

use crate::{
  error::MouseParseError,
  types::{Button, MouseEvent, ScrollDirection},
};

/// Parse a button with support for common aliases
pub fn parse_button_with_aliases(s: &str) -> Result<Button, MouseParseError> {
  let s = s.to_lowercase();
  match s.as_str() {
      "left" | "lmb" => Ok(Button::Left),
      "right" | "rmb" => Ok(Button::Right),
      "middle" | "mmb" | "wheel" => Ok(Button::Middle),
      "x1" | "back" => Ok(Button::X1),
      "x2" | "forward" => Ok(Button::X2),
      "extra3" => Ok(Button::Extra3),
      "extra4" => Ok(Button::Extra4),
      "extra5" => Ok(Button::Extra5),
      "extra6" => Ok(Button::Extra6),
      "extra7" => Ok(Button::Extra7),
      "extra8" => Ok(Button::Extra8),
      _ => Err(MouseParseError::UnknownButton(s)),
  }
}

/// Parse a scroll direction from a string
pub fn parse_scroll_direction(s: &str) -> Result<ScrollDirection, MouseParseError> {
  let s = s.to_lowercase();
  match s.as_str() {
      "verticalup" | "up" => Ok(ScrollDirection::VerticalUp),
      "verticaldown" | "down" => Ok(ScrollDirection::VerticalDown),
      "horizontalleft" | "left" => Ok(ScrollDirection::HorizontalLeft),
      "horizontalright" | "right" => Ok(ScrollDirection::HorizontalRight),
      _ => Err(MouseParseError::UnknownButton(s)),
  }
}

/// Parse a mouse event from a string representation
/// Supports formats like:
/// - "Press(Left)"
/// - "Release(Right)"
/// - "Scroll(VerticalUp, 3)"
/// - "Move(100, 200)"
/// - "RelativeMove(5, -3)"
pub fn parse_mouse_input(s: &str) -> Result<MouseEvent, MouseParseError> {
  if s.is_empty() {
      return Err(MouseParseError::EmptyInput);
  }

  let (event_type, params) = s.split_once('(')
      .and_then(|(t, p)| p.strip_suffix(')').map(|p| (t, p)))
      .ok_or_else(|| MouseParseError::UnknownButton(s.to_string()))?;

  match event_type {
      "Press" => {
          let button = parse_button_with_aliases(params)?;
          Ok(MouseEvent::Press(button))
      }
      "Release" => {
          let button = parse_button_with_aliases(params)?;
          Ok(MouseEvent::Release(button))
      }
      "Scroll" => {
          let parts: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
          if parts.len() != 2 {
              return Err(MouseParseError::UnknownButton(s.to_string()));
          }
          let dir = parse_scroll_direction(parts[0])?;
          let amount = parts[1].parse().map_err(|_| MouseParseError::UnknownButton(s.to_string()))?;
          Ok(MouseEvent::Scroll(dir, amount))
      }
      "Move" => {
          let parts: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
          if parts.len() != 2 {
              return Err(MouseParseError::UnknownButton(s.to_string()));
          }
          let x = parts[0].parse().map_err(|_| MouseParseError::UnknownButton(s.to_string()))?;
          let y = parts[1].parse().map_err(|_| MouseParseError::UnknownButton(s.to_string()))?;
          Ok(MouseEvent::Move(x, y))
      }
      "RelativeMove" => {
          let parts: Vec<&str> = params.split(',').map(|s| s.trim()).collect();
          if parts.len() != 2 {
              return Err(MouseParseError::UnknownButton(s.to_string()));
          }
          let dx = parts[0].parse().map_err(|_| MouseParseError::UnknownButton(s.to_string()))?;
          let dy = parts[1].parse().map_err(|_| MouseParseError::UnknownButton(s.to_string()))?;
          Ok(MouseEvent::RelativeMove(dx, dy))
      }
      _ => Err(MouseParseError::UnknownButton(s.to_string())),
  }
}