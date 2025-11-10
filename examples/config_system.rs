//! 配置系统示例
//!
//! 展示如何使用 mouse-codes 处理用户配置和按键绑定
//! 注意：运行此示例需要启用 serde 特性：
//! cargo run --example config_system --features serde

use mouse_codes::{parse_button_ignore_case, Button};
use std::collections::HashMap;

// 只在启用 serde 特性时编译这部分代码
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    mouse_bindings: HashMap<String, String>,
    sensitivity: f32,
}

#[cfg(not(feature = "serde"))]
#[derive(Debug)]
struct AppConfig {
    mouse_bindings: HashMap<String, String>,
    sensitivity: f32,
}

impl AppConfig {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert("primary_action".to_string(), "left".to_string());
        bindings.insert("secondary_action".to_string(), "right".to_string());
        bindings.insert("special_action".to_string(), "x1".to_string());
        bindings.insert("menu_action".to_string(), "middle".to_string());

        Self {
            mouse_bindings: bindings,
            sensitivity: 1.0,
        }
    }

    fn get_button_for_action(&self, action: &str) -> Option<Button> {
        self.mouse_bindings
            .get(action)
            .and_then(|btn_str| parse_button_ignore_case(btn_str).ok())
    }

    fn validate_bindings(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        for (action, btn_str) in &self.mouse_bindings {
            if parse_button_ignore_case(btn_str).is_err() {
                errors.push(format!("无效的按钮绑定: {} -> {}", action, btn_str));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn print_bindings(&self) {
        println!("鼠标灵敏度: {}", self.sensitivity);
        println!("按键绑定:");
        for (action, button_str) in &self.mouse_bindings {
            if let Ok(button) = parse_button_ignore_case(button_str) {
                println!("  {} -> {} ({})", action, button, button_str);
            } else {
                println!("  {} -> {} (无效)", action, button_str);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 检查是否启用了 serde 特性
    #[cfg(not(feature = "serde"))]
    {
        println!("注意: 此示例需要启用 serde 特性");
        println!("请使用以下命令运行: cargo run --example config_system --features serde");
        return Ok(());
    }

    #[cfg(feature = "serde")]
    {
        use serde_json;

        // 加载默认配置
        let config = AppConfig::default();
        println!("默认配置:");
        config.print_bindings();

        // 验证配置
        match config.validate_bindings() {
            Ok(()) => println!("\n✅ 配置验证通过"),
            Err(errors) => {
                println!("\n❌ 配置错误:");
                for error in errors {
                    println!("  - {}", error);
                }
            }
        }

        // 演示从字符串加载配置
        println!("\n--- 从 JSON 加载配置 ---");
        let config_json = r#"
        {
            "mouse_bindings": {
                "attack": "left",
                "block": "right", 
                "dodge": "x1",
                "interact": "x2",
                "map": "middle"
            },
            "sensitivity": 1.5
        }
        "#;

        let user_config: AppConfig = serde_json::from_str(config_json)?;
        user_config.print_bindings();

        // 演示查询操作对应的按钮
        println!("\n--- 查询操作绑定 ---");
        let actions = ["attack", "block", "dodge", "map"];
        for action in actions {
            if let Some(button) = user_config.get_button_for_action(action) {
                println!("{} -> {}", action, button);
            } else {
                println!("{} -> 未绑定", action);
            }
        }

        // 演示错误配置
        println!("\n--- 错误配置演示 ---");
        let bad_config_json = r#"
        {
            "mouse_bindings": {
                "valid_action": "left",
                "invalid_action": "invalid_button"
            },
            "sensitivity": 1.0
        }
        "#;

        let bad_config: AppConfig = serde_json::from_str(bad_config_json)?;
        match bad_config.validate_bindings() {
            Ok(()) => println!("✅ 配置验证通过"),
            Err(errors) => {
                println!("❌ 配置错误:");
                for error in errors {
                    println!("  - {}", error);
                }
            }
        }
    }

    Ok(())
}
