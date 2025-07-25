fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter*2; // break 跳出循环的同时也可以返回值
        }
    };
    println!("The result is {}", result);

    // 使用 Range 来生成从一个数字开始到另一个数字之前结束的所有数字的序列，循环执行代码特定次数
    for number in 1..4 {
        println!("{}!", number);
    }
}
