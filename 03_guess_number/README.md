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

## 循环

loop

在 rust 中，loop 关键字用于创建一个无限循环

```rust
loop {
    // 循环体
}
```

其中，我们可以使用continue 和 break 来控制循环的执行，这里的逻辑和其他语言一样。

- continue 用于跳过当前循环的剩余部分，直接进入下一次循环
- break 用于终止循环

## 错误处理

在 Rust 中，错误处理通常使用 Result 枚举。Result 有两个变体：Ok 和 Err。Ok 用于表示成功的结果，Err 用于表示错误。

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

- 这里我们尝试将用户输入的字符串转换为 u32 类型。如果转换成功，我们将结果存储在 num 中；如果转换失败，我们使用 continue 跳过当前循环，提示用户重新输入。

## match 表达式

match 关键字用于模式匹配。它类似于其他语言中的 switch 语句，但更强大。

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;
    }
}
```

match 返回的类型为 Ordering 枚举的一个变体，根据比较结果执行不同的代码块。

- Less 表示猜测的数字小于目标数字
- Greater 表示猜测的数字大于目标数字
- Equal 表示猜测的数字等于目标数字

当猜测正确时，打印 "You win!" 并使用 break 终止循环。

## shadowing

变量遮蔽（shadowing）是指在同一作用域内，可以使用相同的变量名来声明一个新的变量，这个新变量会遮蔽掉之前的变量。

```rust
let mut guess = String::new();

std::io::stdin().read_line(&mut guess).expect("Failed to read line");

let guess = guess.trim().parse().expect("Please type a number!");
// 这里，第一次声明的 guess 是一个可变的字符串，用于存储用户输入。第二次声明的 guess 使用了相同的变量名，但它是一个不可变的 u32 类型，存储了用户输入的数字形式。
```

通过这种方式，我们可以在不需要额外变量名的情况下，转换变量的类型或值。
