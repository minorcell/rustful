const MAX_POINT: u32 = 100_000;

fn main() {
    println!("常量MAX_POINT is {}", MAX_POINT);
    let mut x = 5;
    // mut 表示可变，否则不能修改

    println!("the value of x is {}", x);

    x = 6;

    let x = x + 1;
    // shaowding 可以使用相同名字声明新的变量，新的变量会隐藏之前的声明的同名变量
    // 使用 let 生命的新变量，可以和之前同名的变量类型不一致。

    println!("the value of x is {}", x);
}
