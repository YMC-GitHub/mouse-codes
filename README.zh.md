# mouse-codes

[![Crates.io](https://img.shields.io/crates/v/mouse-codes.svg)](https://crates.io/crates/mouse-codes)
[![Documentation](https://docs.rs/mouse-codes/badge.svg)](https://docs.rs/mouse-codes)
[![License](https://img.shields.io/crates/l/mouse-codes.svg)](https://crates.io/crates/mouse-codes)

一个全面的 Rust 库，用于跨平台鼠标按钮代码映射和转换。为 Windows、Linux 和 macOS 提供标准化的鼠标按钮定义和平台特定的代码映射。

## 特性

- **跨平台支持**: 为 Windows、Linux 和 macOS 提供统一的 API
- **标准按钮映射**: 预定义的常用鼠标按钮映射
- **自定义按钮支持**: 可扩展的自定义鼠标按钮系统
- **灵活解析**: 支持不区分大小写的解析和别名
- **多后端支持**: 可选择 `std::collections::HashMap` 或 `phf` 以获得不同性能表现
- **Serde 支持**: 可选的序列化/反序列化支持
- **扩展按钮**: 支持额外的鼠标按钮（功能门控）

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
mouse-codes = "0.1"
```

### 可选功能

- `serde`: 启用序列化支持
- `phf`: 使用完美哈希函数以获得更好性能
- `extended`: 启用额外鼠标按钮支持（Extra9、Extra10）

## 快速开始

```rust
use mouse_codes::{Button, Platform};

fn main() {
    // 获取当前平台上左键的代码
    let platform = Platform::current();
    let code = Button::Left.to_code(platform);
    println!("在 {} 上左键的代码: {}", platform, code);

    // 从字符串解析按钮
    let button: Button = "Right".parse().unwrap();
    println!("解析的按钮: {}", button);

    // 将代码转换回按钮
    if let Some(btn) = Button::from_code(1, Platform::Windows) {
        println!("在 Windows 上代码 1 对应: {}", btn);
    }
}
```

## 使用示例

### 基本按钮操作

```rust
use mouse_codes::{Button, Platform};

// 将按钮转换为平台特定代码
let windows_code = Button::Left.to_code(Platform::Windows); // 1
let linux_code = Button::Right.to_code(Platform::Linux);    // 3
let macos_code = Button::Middle.to_code(Platform::MacOS);   // 2

// 将代码转换回按钮
let button1 = Button::from_code(1, Platform::Windows); // Some(Button::Left)
let button2 = Button::from_code(8, Platform::Linux);   // Some(Button::X1)
```

### 字符串解析

```rust
use mouse_codes::{parse_button_ignore_case, parse_button_with_aliases};

// 不区分大小写的解析
let button1 = parse_button_ignore_case("left").unwrap();    // Button::Left
let button2 = parse_button_ignore_case("MIDDLE").unwrap();  // Button::Middle

// 使用别名解析
let button3 = parse_button_with_aliases("lmb").unwrap();    // Button::Left
let button4 = parse_button_with_aliases("back").unwrap();   // Button::X1
```

### 自定义按钮映射

```rust
use mouse_codes::{CustomButton, CustomButtonMap, Platform};

let mut custom_map = CustomButtonMap::new("我的自定义映射");

// 添加带有平台特定代码的自定义按钮
custom_map.add_button(
    CustomButton::custom_static("超级点击"),
    Some(16),  // Windows 代码
    Some(18),  // Linux 代码  
    Some(13),  // macOS 代码
).unwrap();

// 使用自定义映射
let code = custom_map.get_code_for_button(
    &CustomButton::custom_static("超级点击"), 
    Platform::Windows
); // Some(16)

let button = custom_map.from_code(16, Platform::Windows); // Some(CustomButton::Custom(...))
```

### 鼠标事件解析

```rust
use mouse_codes::parse_mouse_input;

let event1 = parse_mouse_input("Press(Left)").unwrap();
let event2 = parse_mouse_input("Scroll(VerticalUp, 5)").unwrap();
let event3 = parse_mouse_input("Move(100, 200)").unwrap();
```

## 平台映射表

| 按钮      | Windows | Linux | macOS |
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

## 性能

该库提供两种实现后端：

- **HashMap**（默认）：使用 `std::collections::HashMap`
- **PHF**：使用完美哈希函数进行编译时优化的查找

启用 `phf` 功能以获得更好性能：

```toml
[dependencies]
mouse-codes = { version = "0.1", features = ["phf"] }
```

## 许可证

根据以下任一许可证授权：

- Apache 许可证 2.0 版（[LICENSE-APACHE](LICENSE-APACHE) 或 http://www.apache.org/licenses/LICENSE-2.0）
- MIT 许可证（[LICENSE-MIT](LICENSE-MIT) 或 http://opensource.org/licenses/MIT）

按您的选择。

## 贡献

除非您明确声明，否则根据 Apache-2.0 许可证定义，您有意提交用于包含在本作品中的任何贡献均应按上述双重许可，无需任何附加条款或条件。
