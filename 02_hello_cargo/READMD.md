# 使用 cargo 来管理 Rust 项目

## cargo

cargo 是 Rust 的构建系统和包管理工具

- 构建代码
- 下载依赖的库
- 构建这些库

安装 Rust 时会自动安装 cargo

- 运行 `cargo --version` 来查看是否安装成功

```bash
➜  ~ cargo --version
cargo 1.92.0 (344c4567c 2025-10-21)
```

## 创建新项目

使用 `cargo new` 命令来创建一个新的 Rust 项目

```bash
➜  rustful git:(main) ✗ cd 02_hello_cargo
➜  02_hello_cargo git:(main) ✗ cargo new hello
    Creating binary (application) `hello` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

这会创建一个名为 `hello` 的新目录，里面包含一个简单的 Rust 项目结构

```bash
➜  02_hello_cargo git:(main) ✗ ls hello
Cargo.toml  src
```

- `Cargo.toml` 是项目的配置文件
- `src` 目录包含项目的源代码

### toml 文件

TOML（Tom's Obvious, Minimal Language）是一种用于配置文件的标记语言，`Cargo.toml` 文件使用 TOML 格式来定义项目的元数据和依赖关系

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2024"

[dependencies]
```

- package 部分定义了项目的名称、版本和 Rust 版本
- dependencies 部分用于列出项目所依赖的外部库

> 在 rust 项目中，我们通常把包称为 crate

### src 目录

`src` 目录包含项目的源代码，默认情况下会有一个 `main.rs` 文件

```bash
➜  02_hello_cargo git:(main) ✗ ls hello/src
main.rs
```

## 构建 cargo 项目

cargo build

- 创建可执行文件：target/debug/hello（默认构建为 debug 版本）

第一次运行 cargo build 会在顶层目录生成 cargo.lock 文件

- 该文件负责追踪项目以来的精确版本
- 不需要手动修改改文件
- 类似于 pnpm 的 lock 文件

## 运行 cargo 项目

### cargo run

cargo run 会自动构建项目并运行生成的可执行文件

- 如果之前编译成功过，并且源代码没有改变，那么就会直接运行二进制文件。

```bash
➜  hello git:(main) ✗ cargo run
   Compiling hello v0.1.0 (/Users/mcell/Desktop/rustful/02_hello_cargo/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.77s
     Running `target/debug/hello`
Hello, world!
```

### cargo check

cargo check 会快速检查代码是否有错误，但不会生成可执行文件

> 所以 cargo check 比 cargo build 更快，适合在开发过程中频繁使用

## 发布构建

cargo build --release

- 编译时会进行优化
- 代码会运行更快，但是编译时间更长

- 会在 target/release/ 目录下生成可执行文件，而不是 target/debug/

## 小结

尽量使用 cargo
