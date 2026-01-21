fn main() {
    let number = 10;
    if number > 5 {
        println!("The number is greater than 5");
    } else if number == 5 {
        println!("The number is exactly 5");
    } else {
        println!("The number is 5 or less");
    }

    let greeting = "Hello, world!";

    if greeting.contains("world") {
        println!("{}", greeting);
    } else {
        println!("Greeting does not contain 'world'");
    }

    let sex_input = "M";
    let sex = if sex_input == "M" {
        "Male"
    } else if sex_input == "F" {
        "Female"
    } else {
        "Other"
    };
    println!("Sex: {}", sex);

    // loop
    let mut count = 0;
    let res = loop {
        if count >= 3 {
            break count * 2;
        }
        println!("Count is: {}", count);
        count += 1;
    };

    println!("Loop exited with count: {}", res);

    // while
    let mut num = 0;
    while num < 3 {
        println!("Num is: {}", num);
        num += 1;
    }

    // for
    // "range": 0..3 表示从0到2，不包含右边界
    for i in 0..3 {
        println!("i is: {}", i);
    }

    // "array"
    let arr = [10, 20, 30];
    for value in arr.iter() {
        println!("Array value is: {}", value);
    }
}
