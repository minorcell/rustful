# 猜数字学Rust

## use create

在 rust 文件中，我们引入一个包的方式是使用 use 关键字

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;
```

- use std::io 引入了标准库中的 io 模块，用于处理输入输出
- use rand::Rng 引入了 rand 包中的 Rng trait，用于生成随机数
- use std::cmp::Ordering 引入了标准库中的 cmp 模块中的 Ordering 枚举，用于比较值的顺序

## 不可变的变量

在 Rust 中，变量默认是不可变的。如果你想创建一个可变的变量，需要使用 mut 关键字

```rust
let mut guess = String::new();
```

- 这里我们创建了一个名为 guess 的可变变量，并初始化为一个新的空字符串
