//! Core type definitions for mouse input handling
//!
//! This module contains the fundamental types used throughout the crate,
//! including button definitions, event types, and platform identifiers.

/// Mouse button enumeration and related functionality
pub mod button;
/// Trait definition for code mapping between buttons and platform-specific codes
pub mod code_mapper;
/// Mouse event types and scroll direction definitions
pub mod event;
/// Platform identifiers for cross-platform compatibility
pub mod platform;

pub use button::Button;
pub use code_mapper::CodeMapper;
pub use event::{MouseEvent, ScrollDirection};
pub use platform::Platform;
