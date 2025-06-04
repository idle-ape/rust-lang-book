fn main() {
    let number = 6;

    /*
    过多的 else if 表达式会使代码杂乱无章，所以如果有多于一个 else if 表达式，建议使用 match 分支结构
    */
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    // 在 let 语句中使用 if
    let cond: bool = true;
    let n = if cond { 5 } else { 6 };
    println!("The value of number is: {}", n);
}
