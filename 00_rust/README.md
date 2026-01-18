# Rust 简介

## 为什么要用rust

- rust 是一种令人兴奋的新编程语言，它可以让每个人编写可靠且高效的软件。

- rust 可以用来替换C/C++，Rust 和它们具有同样的性能，但是很多常见的Bug在编译时就可以被消灭。

- rust 是一种通用的编程语言，但是它更擅长以下场景：
  - 需要运行时的速度
  - 需要内存安全
  - 更好的利用多处理器

## 与其他语言的比较

- C/C++ 性能非常好，但类型系统和内存都不太安全
- Jvaa/C#，拥有GC，能保证内存安全也有很多优秀特性，但是性能不行。
- Rust
  - 安全
  - 无需GC
  - 易于维护、调试，代码安全高效

## Rust特别擅长的领域

- 高性能的Web Service
- WebAssembly
- 命令行工具
- 网络编程
- 嵌入式开发
- 系统编程

## Rust 与 Firefox

- Rust 是由 Mozilla 赞助开发的，并且被用于 Firefox 浏览器中，以提高性能和安全性。
- 例如，Firefox 的 CSS 引擎 Stylo 就是用 Rust 编写的
- Rust 的内存安全特性有助于减少浏览器中的内存泄漏和崩溃问题。

## Rust 的用户和案例

- Google：新操作系统 Fuchsia，其中Rust代码大约占了30%
- Amazon：基于Linux开发的直接可以在裸机、虚拟机运行容器的操作系统
- System76: 纯Rust 开发下一代安全的操作系统Redox
- 蚂蚁金服：库操作系统 Occlum
- Cloudflare：使用 Rust 构建高性能的网络服务
- Kong：使用 Rust 开发 API 网关

## Rust的缺点

> 难学；

Rust 中有很多独有的概念，他们和现在大多主流语言都不同。
