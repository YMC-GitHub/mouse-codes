//! 自动化测试工具示例
//!
//! 展示如何使用 mouse-codes 创建自动化鼠标测试脚本

use mouse_codes::{parse_mouse_input, MouseEvent, Platform};
use std::thread;
use std::time::Duration;

struct AutomatedTester {
    platform: Platform,
}

impl AutomatedTester {
    fn new() -> Self {
        Self {
            platform: Platform::current(),
        }
    }

    fn run_test_script(&self, script: &[&str]) {
        println!("开始在 {} 上执行测试脚本", self.platform);

        for (i, command) in script.iter().enumerate() {
            println!("\n步骤 {}: 执行 '{}'", i + 1, command);

            match parse_mouse_input(command) {
                Ok(event) => self.execute_event(event),
                Err(e) => println!("  解析错误: {}", e),
            }

            thread::sleep(Duration::from_millis(1000));
        }

        println!("\n测试脚本执行完成!");
    }

    fn execute_event(&self, event: MouseEvent) {
        match event {
            MouseEvent::Press(button) => {
                let code = button.to_code(self.platform);
                println!("  📥 按下按钮: {} (代码: {})", button, code);
                // 在实际应用中，这里会发送真实的鼠标事件
            }
            MouseEvent::Release(button) => {
                let code = button.to_code(self.platform);
                println!("  📤 释放按钮: {} (代码: {})", button, code);
            }
            MouseEvent::Scroll(direction, amount) => {
                println!("  🖱️ 滚动: {} 距离: {}", direction, amount);
            }
            MouseEvent::Move(x, y) => {
                println!("  🎯 移动到: ({}, {})", x, y);
            }
            MouseEvent::RelativeMove(dx, dy) => {
                println!("  ➡️  相对移动: ({}, {})", dx, dy);
            }
        }
    }
}

fn main() {
    let tester = AutomatedTester::new();

    // 测试脚本 - 模拟用户操作
    let test_script = [
        "Move(100, 200)",
        "Press(Left)",
        "Release(Left)",
        "Scroll(VerticalDown, 5)",
        "RelativeMove(50, 0)",
        "Press(Right)",
        "Release(Right)",
        "Scroll(HorizontalRight, 3)",
    ];

    tester.run_test_script(&test_script);

    // 演示错误处理
    println!("\n--- 错误处理演示 ---");
    let invalid_commands = [
        "Invalid(Command)",
        "Press(InvalidButton)",
        "Scroll(InvalidDirection, 5)",
    ];

    for cmd in invalid_commands {
        match parse_mouse_input(cmd) {
            Ok(event) => println!("'{}' -> {:?}", cmd, event),
            Err(e) => println!("'{}' -> 错误: {}", cmd, e),
        }
    }
}
