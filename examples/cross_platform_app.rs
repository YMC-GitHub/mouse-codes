//! 跨平台应用程序示例
//!
//! 展示如何在不同平台上处理自定义鼠标按钮

use mouse_codes::{Button, CustomButton, CustomButtonMap, Platform};

struct CrossPlatformApp {
    custom_buttons: CustomButtonMap,
    current_platform: Platform,
}

impl CrossPlatformApp {
    fn new() -> Self {
        let mut custom_map = CustomButtonMap::new("AppCustomButtons");

        // 为不同平台定义自定义按钮
        custom_map
            .add_button(
                CustomButton::custom_static("GestureButton"),
                Some(16), // Windows
                Some(18), // Linux
                Some(13), // macOS
            )
            .unwrap();

        custom_map
            .add_button(
                CustomButton::custom_static("QuickMenu"),
                Some(17), // Windows
                Some(19), // Linux
                Some(14), // macOS
            )
            .unwrap();

        Self {
            custom_buttons: custom_map,
            current_platform: Platform::current(),
        }
    }

    fn handle_hardware_event(&self, raw_code: usize) {
        println!(
            "处理硬件事件 - 代码: {}, 平台: {}",
            raw_code, self.current_platform
        );

        // 首先检查自定义按钮
        if let Some(custom_btn) = self
            .custom_buttons
            .from_code(raw_code, self.current_platform)
        {
            match custom_btn {
                CustomButton::Standard(btn) => {
                    self.handle_standard_button(btn);
                }
                CustomButton::Custom(name) => {
                    self.handle_custom_button(&name);
                }
            }
        }
        // 然后检查标准按钮
        else if let Some(btn) = Button::from_code(raw_code, self.current_platform) {
            self.handle_standard_button(btn);
        } else {
            println!("  ❓ 未知按钮代码: {}", raw_code);
        }
    }

    fn handle_standard_button(&self, button: Button) {
        let action = match button {
            Button::Left => "主按钮点击",
            Button::Right => "上下文菜单",
            Button::Middle => "中键点击",
            Button::X1 => "后退",
            Button::X2 => "前进",
            _ => "额外按钮",
        };
        println!("  🖱️  标准按钮: {} -> {}", button, action);
    }

    fn handle_custom_button(&self, name: &str) {
        let action = match name {
            "GestureButton" => "手势按钮激活",
            "QuickMenu" => "快速菜单打开",
            _ => "自定义功能",
        };
        println!("  ⚡ 自定义按钮: {} -> {}", name, action);
    }

    fn print_platform_info(&self) {
        println!("应用程序运行在: {}", self.current_platform);
        println!("配置文件: {}", self.get_platform_specific_config());
    }

    fn get_platform_specific_config(&self) -> String {
        match self.current_platform {
            Platform::Windows => "windows_config.json".to_string(),
            Platform::Linux => "linux_config.json".to_string(),
            Platform::MacOS => "macos_config.json".to_string(),
        }
    }
}

fn main() {
    let app = CrossPlatformApp::new();

    app.print_platform_info();
    println!();

    // 测试各种按钮代码
    let test_codes = vec![
        1,  // 左键
        2,  // 右键
        5,  // X1 (Windows)
        16, // 自定义手势按钮 (Windows)
        17, // 自定义快速菜单 (Windows)
        99, // 未知代码
    ];

    for code in test_codes {
        app.handle_hardware_event(code);
    }

    // 显示所有平台上的按钮映射
    println!("\n--- 跨平台按钮映射 ---");
    let buttons = [Button::Left, Button::Right, Button::X1, Button::X2];

    for button in buttons {
        println!("{}:", button);
        println!("  Windows: {}", button.to_code(Platform::Windows));
        println!("  Linux:   {}", button.to_code(Platform::Linux));
        println!("  macOS:   {}", button.to_code(Platform::MacOS));
    }
}
