# 控制流

## if/else 语句

在 rust 中，`if` 语句用于根据条件执行不同的代码块。基本语法如下：

```rust
if condition {
    // 条件为 true 时执行的代码
} else {
    // 条件为 false 时执行的代码
}
```

- `condition`：一个返回布尔值（`true` 或 `false`）的表达式。

例如：

```rust
let number = 10;
if number < 5 {
    println!("number 小于 5");
} else {
    println!("number 大于或等于 5");
}
```

## else if 语句

`else if` 语句用于在多个条件之间进行选择。基本语法如下：

```rust
if condition1 {
    // 条件1为 true 时执行的代码
} else if condition2 {
    // 条件2为 true 时执行的代码
} else {
    // 所有条件均为 false 时执行的代码
}
```

## 作为表达式的 if

在 Rust 中，`if` 语句可以作为表达式使用，这意味着它可以返回一个值。例如：

```rust
let number = 10;
let result = if number < 5 {
    "小于 5"
} else {
    "大于或等于 5"
};
println!("结果是: {}", result);
```

## 循环语句

Rust 提供了多种循环语句来重复执行代码块，主要包括 `loop`、`while` 和 `for` 循环。

### loop 循环

`loop` 循环用于无限循环，直到显式地使用 `break` 语句退出循环。基本语法如下：

```rust
loop {
    // 循环体
    if condition {
        break; // 退出循环
    }
}
```

例如：

```rust
let mut count = 0;
loop {
    count += 1;
    if count == 5 {
        break;
    }
    println!("count: {}", count);
}
```

### loop 循环返回值

`loop` 循环可以返回一个值，通过在 `break` 语句中指定返回值。基本语法如下：

```rust
let result = loop {
    // 循环体
    if condition {
        break value; // 退出循环并返回 value
    }
};
```
例如：

```rust
let mut count = 0;
let result = loop {
    count += 1;
    if count == 5 {
        break count * 2; // 返回 count * 2
    }
};
println!("结果是: {}", result); // 输出结果是: 10
```

### while 循环

`while` 循环根据条件重复执行代码块。基本语法如下：

```rust
while condition {
    // 循环体
}
```

例如：

```rust
let mut number = 3;
while number != 0 {
    println!("number: {}", number);
    number -= 1;
}
```

### for 循环

`for` 循环用于遍历集合（如数组、向量等）中的元素。基本语法如下：

```rust
for element in collection {
    // 使用 element
}
```

例如：

```rust
let array = [10, 20, 30, 40, 50];
for &item in array.iter() {
    println!("item: {}", item);
}
```

### 范围循环

Rust 提供了范围（range）语法，可以方便地生成一系列数字，用于 `for` 循环。基本语法如下：

```rust
for number in start..end {
    // 使用 number
}
```
例如：

```rust
for number in 1..5 {
    println!("number: {}", number); // 输出 1 到 4
}
```

## 注意事项

- `if` 语句的条件必须是布尔类型。
- `loop` 循环会无限执行，除非使用 `break` 语句退出。
- `while` 循环在条件为 `true` 时执行循环体。
- `for` 循环适用于遍历集合和范围。
- Rust 的循环语句可以嵌套使用。
- 使用范围语法时，`start..end` 表示从 `start` 到 `end`（不包括 `end`）。
- 可以使用 `..=` 语法来包含结束值，例如 `1..=5` 包含 1 到 5。
