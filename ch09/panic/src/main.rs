use core::error;
use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    /*
    panic时程序会崩溃并退出，默认将panic时的栈展开，这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作。
    如果需要项目的二进制文件越小越好，panic时通过在 Cargo.toml 的 [profile] 部分增加 panic='abort'，可以由展开切换为终止。
    */
    // panic!("crash and burn");

    let v = vec![1, 2, 3];
    // v[99]; // 访问超过 vector 结尾的元素，这也好造成panic: index out of bounds

    /*
    使用 Result 来处理可恢复的错误，标准库里 Result 的定义如下：
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    T 和 E 是泛型类型参数

    以下代码可以有更简洁的写法:
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
    } });
    */
    let f = File::open("../hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("../hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Create file err: {:?}", e),
            },
            other_error => panic!("Open file err: {:?}", other_error),
        },
    };

    /*
    失败时 panic 的简写：unwarp 和 expect
    */
    // 如果 Result 的值时Ok，unwrap会返回 Ok 中的值；如果 Result 的值是 Err，unwrap会自动调用panic!
    let f = File::open("./main.rs").unwrap();

    // expect 允许指定 panic 时的错误信息
    let f = File::open("./test.txt").expect("Failed to open the file");

    let ret = read_username_from_file_v2();
    if let Ok(username) = ret {
        println!("read username succ, {}", username);
    }
}

// 传播错误而不是直接panic
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 上述传播错误的模式很常见，所以 Rust 提供了 ? 运算符来使其更易于处理
fn read_username_from_file_v2() -> Result<String, io::Error> {
    /*
    如果 Result 的值是 Ok ，这个表达式将会返回 Ok 中的值而程序将继续执行。
    如果值是 Err ， Err 中的值将作为整个函数的返回值，就好像使用了 return 关键字一样，这样错误值就被传播给了调用者。

    更简洁的写法，在 ? 之后直接使用链式方法调用来进一步缩短代码：
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
    */
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
