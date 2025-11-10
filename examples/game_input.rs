//! 游戏输入处理示例
//!
//! 展示如何在游戏中使用 mouse-codes 处理鼠标输入绑定

use mouse_codes::{parse_button_with_aliases, Button, Platform};
use std::collections::HashMap;

struct GameInput {
    key_bindings: HashMap<String, Button>,
    platform: Platform,
}

impl GameInput {
    fn new() -> Self {
        let mut key_bindings = HashMap::new();
        key_bindings.insert("attack".to_string(), Button::Left);
        key_bindings.insert("aim".to_string(), Button::Right);
        key_bindings.insert("reload".to_string(), Button::X1);
        key_bindings.insert("switch_weapon".to_string(), Button::X2);

        Self {
            key_bindings,
            platform: Platform::current(),
        }
    }

    fn handle_mouse_event(&self, button_code: usize) -> Option<&str> {
        if let Some(button) = Button::from_code(button_code, self.platform) {
            match button {
                Button::Left => Some("攻击"),
                Button::Right => Some("瞄准"),
                Button::X1 => Some("换弹"),
                Button::X2 => Some("切换武器"),
                _ => None,
            }
        } else {
            None
        }
    }

    fn rebind_key(
        &mut self,
        action: &str,
        button_str: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let button = parse_button_with_aliases(button_str)?;
        self.key_bindings.insert(action.to_string(), button);
        Ok(())
    }

    fn print_bindings(&self) {
        println!("当前平台: {}", self.platform);
        println!("按键绑定:");
        for (action, button) in &self.key_bindings {
            let code = button.to_code(self.platform);
            println!("  {} -> {} (代码: {})", action, button, code);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game_input = GameInput::new();
    game_input.print_bindings();

    // 模拟处理鼠标事件
    println!("\n处理鼠标事件:");
    let test_codes = [1, 2, 5, 6]; // 左键、右键、X1、X2 的 Windows 代码
    for &code in &test_codes {
        if let Some(action) = game_input.handle_mouse_event(code) {
            println!("  代码 {}: {}", code, action);
        }
    }

    // 重新绑定按键
    println!("\n重新绑定 'attack' 到 'middle':");
    game_input.rebind_key("attack", "middle")?;
    game_input.print_bindings();

    Ok(())
}
