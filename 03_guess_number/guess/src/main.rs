use std::io;

fn main() {
    println!("猜数字！");

    println!("猜测一个数字，范围是 1 到 100。");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("读取行失败");

    println!("你猜测的数字是：{}", guess);
}
