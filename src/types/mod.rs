//! Core type definitions for mouse buttons, events, and platforms

pub mod button;
pub mod code_mapper;
pub mod event;
pub mod platform;

pub use button::Button;
pub use code_mapper::CodeMapper;
pub use event::{MouseEvent, ScrollDirection};
pub use platform::Platform;