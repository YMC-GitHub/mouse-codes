# mouse-codes

[![Crates.io](https://img.shields.io/crates/v/mouse-codes.svg)](https://crates.io/crates/mouse-codes)
[![Documentation](https://docs.rs/mouse-codes/badge.svg)](https://docs.rs/mouse-codes)
[![License](https://img.shields.io/crates/l/mouse-codes.svg)](https://crates.io/crates/mouse-codes)

A comprehensive Rust library for cross-platform mouse button code mapping and conversion. Provides standardized mouse button definitions and platform-specific code mappings for Windows, Linux, and macOS.

## Features

- **Cross-Platform Support**: Unified API for Windows, Linux, and macOS
- **Standard Button Mapping**: Pre-defined mappings for common mouse buttons
- **Custom Button Support**: Extensible system for custom mouse buttons
- **Flexible Parsing**: Case-insensitive parsing with alias support
- **Multiple Backends**: Choose between `std::collections::HashMap` or `phf` for performance
- **Serde Support**: Optional serialization/deserialization support
- **Extended Buttons**: Support for extra mouse buttons (feature-gated)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mouse-codes = "0.1"
```

### Optional Features

- `serde`: Enables serialization support
- `phf`: Uses perfect hash functions for better performance
- `extended`: Enables support for extra mouse buttons (Extra9, Extra10)

## Quick Start

```rust
use mouse_codes::{Button, Platform};

fn main() {
    // Get code for left button on current platform
    let platform = Platform::current();
    let code = Button::Left.to_code(platform);
    println!("Left button code on {}: {}", platform, code);

    // Parse button from string
    let button: Button = "Right".parse().unwrap();
    println!("Parsed button: {}", button);

    // Convert code back to button
    if let Some(btn) = Button::from_code(1, Platform::Windows) {
        println!("Code 1 on Windows is: {}", btn);
    }
}
```

## Usage Examples

### Basic Button Operations

```rust
use mouse_codes::{Button, Platform};

// Convert buttons to platform-specific codes
let windows_code = Button::Left.to_code(Platform::Windows); // 1
let linux_code = Button::Right.to_code(Platform::Linux);    // 3
let macos_code = Button::Middle.to_code(Platform::MacOS);   // 2

// Convert codes back to buttons
let button1 = Button::from_code(1, Platform::Windows); // Some(Button::Left)
let button2 = Button::from_code(8, Platform::Linux);   // Some(Button::X1)
```

### String Parsing

```rust
use mouse_codes::{parse_button_ignore_case, parse_button_with_aliases};

// Case-insensitive parsing
let button1 = parse_button_ignore_case("left").unwrap();    // Button::Left
let button2 = parse_button_ignore_case("MIDDLE").unwrap();  // Button::Middle

// Parsing with aliases
let button3 = parse_button_with_aliases("lmb").unwrap();    // Button::Left
let button4 = parse_button_with_aliases("back").unwrap();   // Button::X1
```

### Custom Button Mapping

```rust
use mouse_codes::{CustomButton, CustomButtonMap, Platform};

let mut custom_map = CustomButtonMap::new("MyCustomMap");

// Add a custom button with platform-specific codes
custom_map.add_button(
    CustomButton::custom_static("HyperClick"),
    Some(16),  // Windows code
    Some(18),  // Linux code  
    Some(13),  // macOS code
).unwrap();

// Use the custom mapping
let code = custom_map.get_code_for_button(
    &CustomButton::custom_static("HyperClick"), 
    Platform::Windows
); // Some(16)

let button = custom_map.from_code(16, Platform::Windows); // Some(CustomButton::Custom(...))
```

### Mouse Event Parsing

```rust
use mouse_codes::parse_mouse_input;

let event1 = parse_mouse_input("Press(Left)").unwrap();
let event2 = parse_mouse_input("Scroll(VerticalUp, 5)").unwrap();
let event3 = parse_mouse_input("Move(100, 200)").unwrap();
```

## Platform Mappings

| Button    | Windows | Linux | macOS |
|-----------|---------|-------|-------|
| Left      | 1       | 1     | 0     |
| Right     | 2       | 3     | 1     |
| Middle    | 4       | 2     | 2     |
| X1        | 5       | 8     | 3     |
| X2        | 6       | 9     | 4     |
| Extra3    | 7       | 10    | 5     |
| Extra4    | 8       | 11    | 6     |
| Extra5    | 9       | 12    | 7     |
| Extra6    | 10      | 13    | 8     |
| Extra7    | 11      | 14    | 9     |
| Extra8    | 12      | 15    | 10    |

## Performance

The crate offers two implementation backends:

- **HashMap** (default): Uses `std::collections::HashMap`
- **PHF**: Uses perfect hash functions for compile-time optimized lookups

Enable the `phf` feature for better performance:

```toml
[dependencies]
mouse-codes = { version = "0.1", features = ["phf"] }
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.