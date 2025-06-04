fn main() {
    // loop 循环
    let mut counter = 0;
    loop {
        println!("again");
        counter = counter + 1;
        if counter == 3 {
            break;
        }
    }

    let mut c = 0;
    let result = loop {
        c = c + 1;
        if c == 10 {
            // 跳出循环的同时，返回一个值，这个语法有点意思，C和go中没有见过。
            break c * 2;
        }
    };
    println!("The result is {}", result);

    // while 循环
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");

    // for 循环
    // 遍历集合
    let a = [10, 20, 30, 40, 50];
    for ele in a.iter() {
        println!("the value is {}", ele);
    }
    // 执行特定次数，(1..4)生成的是一个range，rev用来反转 range
    for n in (1..4).rev() {
        println!("{}!", n);
    }
    println!("LIFTOFF!!!");
}
