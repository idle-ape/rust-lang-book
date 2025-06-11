fn main() {
    // 创建一个空的 String
    let mut s = String::new();
    // 使用 to_string() 方法从字符串字面值创建 String
    let s = "initial_contents".to_string();
    // 使用 String::from 函数来从字符串字面值创建 String
    let s = String::from("initial contents"); // 等同于 to_string()

    // 使用 push_str 方法附加字符串
    let mut s = String::from("foo");
    println!("origin s = {}", s);
    s.push_str("bar");
    println!("after push s = {}", s);
    // 使用 push 方法附加字符
    let mut s = String::from("lo");
    s.push('l');
    println!("s = {}", s);

    /*
    使用 + 来拼接字符串，+ 号拼接等同于调用以下函数：
    fn add(self, s: &str) -> String {}
    第一个参数表明该函数获取了 s1 的所有权，而第二个参数是个引用，只是借用了 s2 的值，所以 s1 不能再继续用了
    */
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能再继续使用

    // 使用 format! 宏来拼接字符串
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s = {}", s);

    /*
    rust 的字符串不支持索引
    String是一个 Vec<u8> 的封装。索引操作预期总是需要常数时间 (O(1))。
    但是对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符。
    */
    let string = String::from("hello");
    // let h = string[0]; // the type `str` cannot be indexed by `{integer}`

    /*
    遍历字符串
    char h in hello
    char e in hello
    char l in hello
    char l in hello
    char o in hello
    */
    for c in string.chars() {
        println!("char {} in {}", c, string);
    }
    match string.chars().nth(0) {
        Some(c) => println!("first char of {} is {}", string, c),
        None => println!("first char not found"),
    }
}
