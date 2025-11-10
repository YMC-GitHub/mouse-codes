## 几种 traits 的位置与名称方案
- 使用 core 替换 traits 作为模块名
- 使用 interfaces 替换 traits 作为模块名
- traits 直接放在 lib.rs 和相关模块中
- types vs core vs traits
- 此处 types ， core ， interfaces 的含义

<!-- types=enum,struct -->
<!-- interfaces=trait -->
<!-- core:enum,struct,trait,fn,const -->


## 文档生成 | 1
- 为 crate 生成英文版 README.en.md
- 为 crate 生成中文版 README.md

## 文档生成 | 2
- 为 crate 生成中文版 README.zh.md
- 为 crate 生成英文版 README.md

## 文档生成 | 3
- 源码文件中使用 英文注释 用于生成 crate.io 文档
- 文档注释是否符合 crate.io 规范


## 文档生成 | 4
- 源码文件中使用 中文注释 用于生成 crate.io 文档


## 模块分析
- 分析模块依赖关系图，是否存在循环依赖

## crate.io  规范
- 检查 keywords 是否在5个以内
- 检查 categories 是否符合 crate.io 规范


## 项目结构
```plaintext
mouse-codes/
├── src/
│   ├── lib.rs              # 入口文件，类型重导出
│   ├── error.rs            # 错误处理
│   ├── types/              # 核心类型定义
│   │   ├── mod.rs
│   │   ├── button.rs       # 鼠标按钮枚举
│   │   ├── event.rs        # 鼠标事件类型
│   │   ├── platform.rs     # 平台枚举
│   │   └── code_mapper.rs  # 编码映射 trait
│   ├── mapping/            # 映射实现
│   │   ├── mod.rs
│   │   ├── standard.rs     # 标准映射
│   │   └── custom.rs       # 自定义映射
│   ├── parser/             # 解析功能
│   │   └── mod.rs
│   └── utils.rs            # 工具函数
├── examples/               # 使用示例
├── Cargo.toml
└── README.md
```

```
src/mapping/
├── standard/
│   ├── mod.rs          # 公共接口和特性分发
│   ├── phf_impl.rs     # 启用 phf 特性时的实现
│   └── hashmap_impl.rs # 不启用 phf 特性时的实现
└── mod.rs
```

## 目录创建
```powershell
# 1.0
# sh -c "touch src/{lib,error,types,mapping,utils}.rs"

# 1.1
sh -c "touch src/{lib,error,utils}.rs"

sh -c "mkdir -p src/types"
# sh -c "touch src/types/{mod,key,modifier,platform,code_mapper}.rs"
sh -c "touch src/types/{mod,button,event,platform,code_mapper}.rs"

sh -c "mkdir -p src/mapping"
sh -c "touch src/mapping/{mod,standard,custom}.rs"

sh -c "mkdir -p src/mapping/standard"
sh -c "touch src/mapping/standard/{mod,phf_impl,hashmap_impl}.rs"
# sh -c "rm src/mapping/*todel*"

sh -c "mkdir -p examples"
# sh -c "touch examples/{coordinates_demo,bit_ops_demo,messages_demo,full_demo}.rs"
# sh -c "touch examples/basic_usage.rs"
sh -c "touch examples/{basic_usage,game_input,automated_testing,cross_platform_app,config_system}.rs"


# sh -c "rm -r examples"

sh -c "touch src/{lib,types,error}.rs"
sh -c "mkdir -p src/core"
sh -c "touch src/core/{mod,window_target,global}.rs"


sh -c "mkdir -p src/parser"
# sh -c "touch src/parser/{mod,command,duration}.rs"
sh -c "touch src/parser/mod.rs"



sh -c "mkdir -p src/convenience"
sh -c "touch src/convenience/{mod,shortcuts}.rs"

sh -c "mkdir -p src/smart"
sh -c "touch src/smart/{mod,implementation}.rs"


# sh -c "rm -r src/convenience"
# sh -c "rm src/parser/duration.rs"

# 检查项目结构
cargo check

# 运行测试
cargo test

# 构建项目
cargo build

# 构建文档
cargo doc --open

# cargo build > src_build_log.txt 2>&1

# extract the core module while maintaining interface compatibility and preserving the original implementation

git add .
git commit -m "refactor:extract core module"

```

## 接口优化建议 | 1
- 是否应该 将  `current_platform()` 实现 改写到 `Platform::current()`
- 有必要支持custom按键吗？若有必要，有哪些使用场景

## 模块依赖关系
- 分析当前模块之间的依赖关系图，是否存在循环依赖

## 代码未用警告
- 当前模块是否存在未使用的函数或变量
```powershell
# 检查未使用代码
cargo check
cargo clippy -- -W unused

# 检查特定特性组合
cargo check --no-default-features
cargo check --features "sorting"
cargo check --features "selection" 
cargo check --features "sorting,selection"
```

## 代码注释输出
- 检查代码注释是否符合文档规范

- 在文档注释中添加简单示例代码
- 文档注释使用英文
- 非文档注释使用中文
- 在console中输出信息使用英文

```powershell

# 生成本地文档并在浏览器中打开
cargo doc --open --all-features

# 或者只生成文档不打开
cargo doc --all-features

# 检查文档是否完整
cargo doc --no-deps --all-features

# 运行文档测试
cargo test --doc

# 检查是否有缺失的文档
cargo doc --no-deps --all-features --message-format=short 2>&1 | grep "missing documentation"

# 运行所有文档测试
cargo test --doc --all-features

# 检查链接是否有效
cargo doc --no-deps --all-features --document-private-items
```

```powershell

# 首先验证包
cargo package

# 然后发布
cargo publish


```

## 代码质量 | 格式化
```powershell
# 查看 rustfmt 版本和支持的选项
rustfmt --version
rustfmt --help=config

# 格式化整个项目
cargo fmt

# 格式化特定文件
cargo fmt -- src/enumerator.rs

# 检查格式而不修改
cargo fmt -- --check
```


```powershell
cargo clippy -- -D warnings


# cargo clippy --fix --allow-dirty --allow-staged -- -D warnings


```


## 发布之前检查
```powershell
cargo fmt;cargo fmt -- --check

cargo clippy -- -D warnings
cargo clippy --all-features -- -D warnings

cargo test --all-features

cargo doc --no-deps --all-features
# cargo doc --no-deps --all-features --open

cargo package --allow-dirty
cargo publish --dry-run --registry crates-io --allow-dirty

# git push ghg main
# gh workflow run crate-publish-s.yml -f dry_run=true
# gh run list --workflow=crate-publish-s.yml
```

## 运行示例
```powershell
cargo run --example basic_usage > src_demo_log.md 2>&1

# 基础用法
cargo run --example basic_usage

# 游戏输入处理
cargo run --example game_input

# 自动化测试
cargo run --example automated_testing

# 跨平台应用
cargo run --example cross_platform_app

# 配置系统
cargo run --example config_system > src_demo_log.md 2>&1
```