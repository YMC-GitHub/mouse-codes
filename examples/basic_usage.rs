//! 基础用法示例
//!
//! 展示 mouse-codes 的基本功能和 API 使用

use mouse_codes::{parse_button_ignore_case, parse_button_with_aliases, Button, Platform};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== mouse-codes 基础用法示例 ===\n");

    // 1. 平台检测
    let platform = Platform::current();
    println!("1. 当前平台: {}", platform);

    // 2. 按钮到代码的转换
    println!("\n2. 按钮到代码转换:");
    let buttons = [
        Button::Left,
        Button::Right,
        Button::Middle,
        Button::X1,
        Button::X2,
    ];
    for button in buttons {
        let code = button.to_code(platform);
        println!("   {} -> 代码 {}", button, code);
    }

    // 3. 代码到按钮的转换
    println!("\n3. 代码到按钮转换:");
    let codes = [1, 2, 4, 5, 6]; // Windows 平台的代码
    for &code in &codes {
        if let Some(button) = Button::from_code(code, Platform::Windows) {
            println!("   代码 {} -> {}", code, button);
        } else {
            println!("   代码 {} -> 未知按钮", code);
        }
    }

    // 4. 字符串解析
    println!("\n4. 字符串解析:");
    let button_strs = ["Left", "RIGHT", "middle", "X1", "x2"];
    for btn_str in button_strs {
        match Button::from_str(btn_str) {
            Ok(button) => println!("   '{}' -> {}", btn_str, button),
            Err(_) => println!("   '{}' -> 解析失败", btn_str),
        }
    }

    // 5. 大小写不敏感解析
    println!("\n5. 大小写不敏感解析:");
    let case_variants = ["left", "LEFT", "Left", "LeFt"];
    for variant in case_variants {
        match parse_button_ignore_case(variant) {
            Ok(button) => println!("   '{}' -> {}", variant, button),
            Err(e) => println!("   '{}' -> 错误: {}", variant, e),
        }
    }

    // 6. 别名解析
    println!("\n6. 别名解析:");
    let aliases = ["lmb", "rmb", "mmb", "back", "forward"];
    for alias in aliases {
        match parse_button_with_aliases(alias) {
            Ok(button) => println!("   '{}' -> {}", alias, button),
            Err(e) => println!("   '{}' -> 错误: {}", alias, e),
        }
    }

    // 7. 跨平台代码比较
    println!("\n7. 跨平台代码比较:");
    let test_button = Button::Left;
    println!("   {} 按钮在不同平台上的代码:", test_button);
    println!("     Windows: {}", test_button.to_code(Platform::Windows));
    println!("     Linux:   {}", test_button.to_code(Platform::Linux));
    println!("     macOS:   {}", test_button.to_code(Platform::MacOS));

    // 8. 显示所有可用按钮
    println!("\n8. 所有可用按钮:");
    for button in &[
        Button::Left,
        Button::Right,
        Button::Middle,
        Button::X1,
        Button::X2,
        Button::Extra3,
        Button::Extra4,
        Button::Extra5,
        Button::Extra6,
        Button::Extra7,
        Button::Extra8,
    ] {
        println!("   - {}", button);
    }

    Ok(())
}
