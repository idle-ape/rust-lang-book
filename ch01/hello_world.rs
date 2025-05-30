fn main() {
    // println! 是一个 Rust 宏（macro），如果是调用函数，则应该输入 println(没有!)。
    // 当看到符号 ! 的时候，就意味 着调用的是宏而不是普通函数。
    // 和C一样，一行以分号结尾，代表一个表达式的结束和下一个表达式的开始，Go语言通常不需要分号。
    // 
    // 编译：rustc main.rs
    println!("Hello, world!");
}