use rand::Rng; // 引入 rand 库中的 Rng trait，用于生成随机数
use std::cmp; // 引入标准库中的cmp，用于比较
use std::io; // 将 io (输入/输出)库引入当前作用域
fn main() {
    println!("Guess the number!"); // 使用 println! 宏在屏幕上打印字符串

    let secret_number = rand::thread_rng().gen_range(1, 101); // 生成一个随机数，范围为 1 到 100
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess."); // ditto:5

        // 创建一个空的字符串变量 guess，let 用来创建变量，变量默认是不可修改的，前面加 mut 则表示可修改这个变量
        // let foo = 5; // 不可变
        // let mut bar = 5; // 可变
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) // 使用 Result 类型来处理潜在的错误，Result 类型是 枚举(enumerations)，Result 的成员是 Ok 和 Err ， Ok 成员表示操作成功，内部包含成功时产生的值。 Err成员则意味着操作失败，并且包含失败的前因后果。
            .expect("Failed to read line");

        // Rust 允许用一个新值来隐藏 (shadow) guess 之前的值。它允许我们复用 guess 变量的名字，而不是被迫创建两个不同变量，诸如 guess_str 和 guess 之类。
        // 即使两个同名的变量类型不一致也可以？利弊不好说...
        // let guess: u32 = guess.trim().parse().expect("Please enter a valid number!");
        // 当输入无效的数字时使用expect()会导致崩溃，这里改用match对parse返回的两种情况作处理
        let guess: u32 = match guess.trim().parse() {
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
            Ok(num) => num,
        };

        // {} 是预留在特定位置的占位符，使用多个 {} 也可以打印多个值。
        // let x = 5;
        // let y = 10;
        // println!("x = {} and y = {}", x, y);
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
            cmp::Ordering::Greater => println!("Too big!"),
            cmp::Ordering::Less => println!("Too small!"),
        }
    }
}
