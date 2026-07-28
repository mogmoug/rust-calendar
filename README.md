# Rust Calendar

[![Crates.io](https://img.shields.io/crates/v/rust-calendar.svg)](https://crates.io/crates/rust-calendar)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

一个使用 Rust 编写的命令行日历工具，适合初学者学习 Rust 编程。

## 📖 目录

- [功能特性](#-功能特性)
- [快速开始](#-快速开始)
- [使用方法](#-使用方法)
- [项目结构](#-项目结构)
- [代码示例](#-代码示例)
- [编译与安装](#-编译与安装)
- [依赖说明](#-依赖说明)
- [贡献指南](#-贡献指南)
- [许可证](#-许可证)

## ✨ 功能特性

- 📅 显示当前月份的日历
- 🗓️ 查询指定年月的日历
- 🔄 支持自定义每周起始日（周日或周一）
- 🎯 简洁的命令行界面
- 📚 提供库接口，可作为依赖使用

## 🚀 快速开始

### 前置要求

- Rust 1.56+ (Edition 2021)
- Cargo 包管理器

### 安装 Rust

如果您还没有安装 Rust，请访问 [https://rustup.rs](https://rustup.rs) 或使用以下命令：

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# 下载并运行 rustup-init.exe from https://rustup.rs
```

### 克隆项目

```bash
git clone https://github.com/mogmoug/rust-calendar.git
cd rust-calendar
```

### 构建项目

```bash
cargo build --release
```

### 运行

```bash
# 查看当前月份日历
cargo run -- now

# 查看指定年月日历
cargo run -- date 2024 12

# 查看帮助
cargo run -- --help
```

## 💡 使用方法

### 命令行参数

```
A calendar command-line tool written in rust

Usage: rust-calendar [OPTIONS] [COMMAND]

Commands:
  now         获取当前月份的日历
  date        获取指定年月的日历 (用法：rust-calendar date [YEAR] [MONTH])
  help        打印此消息或给定子命令的帮助

Options:
  -t, --the-first-day-of-the-week <THE_FIRST_DAY_OF_THE_WEEK>
          每周的第一天。周日 (0) 或 周一 (1) [默认：0]
  -h, --help
          打印帮助信息
  -V, --version
          打印版本信息
```

### 使用示例

#### 1. 显示当前月份日历

```bash
rust-calendar now
```

输出示例：
```
Sun     Mon     Tue     Wed     Thu     Fri     Sat
        1       2       3       4       5       6
7       8       9       10      11      12      13
14      15      16      17      18      19      20
21      22      23      24      25      26      27
28      29      30      31
```

#### 2. 显示指定年月日历

```bash
rust-calendar date 2024 12
```

输出示例：
```
Sun     Mon     Tue     Wed     Thu     Fri     Sat
1       2       3       4       5       6       7
8       9       10      11      12      13      14
15      16      17      18      19      20      21
22      23      24      25      26      27      28
29      30      31
```

#### 3. 设置每周从周一开始

```bash
rust-calendar -t 1 now
```

输出示例：
```
Mon     Tue     Wed     Thu     Fri     Sat     Sun
1       2       3       4       5       6       7
8       9       10      11      12      13      14
15      16      17      18      19      20      21
22      23      24      25      26      27      28
29      30      31
```

## 📁 项目结构

```
rust-calendar/
├── Cargo.toml              # 项目配置和依赖管理
├── README.md               # 项目说明文档
├── LICENSE                 # MIT 许可证
├── src/
│   ├── main.rs             # 程序入口点
│   ├── lib.rs              # 库模块导出
│   ├── calendar.rs         # 日历核心逻辑
│   ├── options.rs          # 命令行参数解析
│   └── test.rs             # 测试模块
└── examples/
    ├── now.rs              # 示例：显示当前月份
    └── print_calendar.rs   # 示例：打印全年日历
```

### 模块说明

| 文件 | 描述 |
|------|------|
| `main.rs` | 程序主入口，处理命令行参数并调用相应功能 |
| `lib.rs` | 库模块定义，导出公共 API |
| `calendar.rs` | 核心日历逻辑，包括日期计算和显示 |
| `options.rs` | 使用 clap 库解析命令行参数 |
| `test.rs` | 单元测试和集成测试 |

## 📝 代码示例

### 作为库使用

您可以在自己的项目中将 `rust-calendar` 作为依赖使用：

```toml
# Cargo.toml
[dependencies]
rust-calendar = "0.1.2"
```

#### 示例 1：显示当前月份

```rust
use rust_calendar::calendar::Calendar;

fn main() {
    Calendar::from_now(
        rust_calendar::calendar::WeekStartingFrom::StartingFromSunday
    ).print();
}
```

#### 示例 2：显示指定年月

```rust
use rust_calendar::calendar::{Calendar, WeekStartingFrom};

fn main() {
    let year = 2024;
    let month = 12;
    
    Calendar::from_year_month(
        WeekStartingFrom::StartingFromMonday,
        year,
        month
    ).print();
}
```

#### 示例 3：打印全年日历

```rust
use rust_calendar::calendar::{Calendar, WeekStartingFrom};

fn main() {
    const STARTINGSUNDAY: WeekStartingFrom = WeekStartingFrom::StartingFromSunday;
    
    for month in 1..=12 {
        println!("=== {}年{}月 ===", 2024, month);
        Calendar::from_year_month(STARTINGSUNDAY, 2024, month).print();
        println!();
    }
}
```

运行示例：

```bash
# 运行示例代码
cargo run --example now
cargo run --example print_calendar
```

## 🔧 编译与安装

### 开发模式编译

```bash
cargo build
```

### 发布模式编译（优化）

```bash
cargo build --release
```

### 运行测试

```bash
cargo test
```

### 格式化代码

```bash
cargo fmt
```

### 代码检查

```bash
cargo clippy
```

### 生成文档

```bash
cargo doc --open
```

### 安装到系统

```bash
cargo install --path .
```

安装后可以直接使用 `rust-calendar` 命令。

## 📦 依赖说明

项目依赖以下 crates：

| 依赖 | 版本 | 用途 |
|------|------|------|
| `chrono` | 0.4.23 | 日期和时间处理 |
| `clap` | 4.0.32 | 命令行参数解析（带 derive 特性） |

### 依赖说明

- **chrono**: Rust 的日期和时间库，用于获取当前日期、计算月份天数等
- **clap**: 现代化的命令行参数解析库，提供 derive 宏简化参数定义

## 🤝 贡献指南

欢迎贡献代码！请遵循以下步骤：

1. Fork 本仓库
2. 创建您的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交您的更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启一个 Pull Request

### 开发建议

- 遵循 Rust 代码规范
- 添加适当的注释
- 为新功能编写测试
- 确保 `cargo test` 和 `cargo clippy` 通过

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🔗 相关链接

- [GitHub 仓库](https://github.com/mogmoug/rust-calendar)
- [Crates.io](https://crates.io/crates/rust-calendar)
- [Rust 官方文档](https://doc.rust-lang.org/)
- [Chrono 文档](https://docs.rs/chrono/)
- [Clap 文档](https://docs.rs/clap/)

## 🙏 致谢

感谢所有为这个项目做出贡献的开发者！

---

**作者**: Mogmoug <mogmoug123@outlook.com>

**版本**: 0.1.2
