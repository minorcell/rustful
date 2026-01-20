use rand::Rng;
use std::cmp::Ordering;
use std::io; // 标准输入输出库 // trait 接口

fn main() {
    println!("猜数字！");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("猜测一个数字，范围是 1 到 100。");

        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("读取行失败");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("你赢了！");
                break;
            }
        }
    }
}
