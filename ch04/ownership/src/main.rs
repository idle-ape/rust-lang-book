/*
1. 变量与数据交互的方式(一): 移动
    如果被移动的数据的类型是可 Copy 的，那被移动后可继续使用
    如果被移动的数据的类型是 Drop，那移动之后就失效了
2. 变量与数据交互的方式(二): 克隆
3. 只在栈上的数据: 拷贝
    作为一个通用的规则，任何简单标量值的组合可以是 Copy 的，不需要分配内存或某种形式资源的类型是 Copy 的。
    如下是一些 Copy 的类型:
        所有整数类型，比如 u32。
        布尔类型，bool ，它的值是 true 和 false。
        所有浮点数类型，比如f64。
        字符类型，char。
        元组，当且仅当其包含的类型也都是 Copy 的时候。比如， (i32, i32) 是 Copy 的，但 (i32, String) 就不是。
    Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait。
4. 引用的规则
    在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
        let mut s = String::from("hello");
        let r1 = &s; // 没问题
        let r2 = &s; // 没问题
        let r3 = &mut s; // 大问题
        println!("{}, {}, and {}", r1, r2, r3);
    引用必须总是有效的。
        fn dangle() -> &String {
            let s = String::from("hello");
            &s
        } // 这里 s 离开作用域并被丢弃，其内存被释放。这是不对的
*/
fn main() {
    // 整数是有已知的固定大小，所以这两个5被放入了栈中，这里是通过拷贝（copy），和下面说的克隆（clone）不同
    let x = 5;
    let y = x;

    /*
    String 由三部分组成（类似于go里面的slice，另外这一组数据存储在栈上）：
    1、ptr：指向存放字符串内存的指针（堆上）
    2、len：当前使用了多少字节的内存，即字符串长度
    3、cap：申请了多少字节的内存
    */
    let s1 = String::from("hello");
    // 如果你在其他语言中听说过术语浅拷贝(shallow copy)和深拷贝(deep copy)，那么拷贝指针、长度和容量而不拷贝数据可能听起来像浅拷贝。
    // 不过因为 Rust 同时使第一个变量无效了，这个操作被称为移动(move)，而不是浅拷贝。
    let s2 = s1;
    // println!("{} {}", s1, s2); // 如果这里还使用s1的话，编译会报错，因为s1已经被移动到s2，s1已经无效了
    println!("{}", s2);

    // 当需要深度复制 String 中堆上的数据，而不仅仅是ptr、len、cap这些栈上的数据，可以通过克隆的方式
    let s1 = String::from("world");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);

    /*
    向函数传递值可能会移动或者复制，就像赋值语句一样。
    */
    let s = String::from("hello"); // s 的作用域开始
    takes_ownership(s); // s的值被移动到函数里，之后s不再有效
    // println!("use s after move, s = {}", s); // 编译报错：borrow of moved value: s
    let x = 5; // x 的作用域开始
    makes_copy(x); // x 的值被移动到函数里，但是 i32 是 Copy 的，所以在后面可以继续使用 x
    println!("use x after move, x = {}", x);

    /*
    返回值也可以转移所有权
    */
    let s1 = gives_ownership(); // gives_ownership 将返回值移给 s1
    let s2 = String::from("good"); // s2 进入作用域
    let s3 = takes_and_gives_back(s2); // s2 被移动到 takes_and_gives_back 中并将返回值移给 s3，之后 s2 无效
    println!("s1 = {}, s3 = {}", s1, s3);

    /*
    使用元组返回多个值
    */
    let (s, len) = calc_length(s3);
    println!("s = {}, len = {}", s, len);

    /*
    以上使用 String 的代码有个弊端，使用完之后原String被移动，导致后面没法使用，除非在被调用的函数里再返回回来
    可以通过引用来解决这个问题：& 符号就是引用，它们允许你使用值但不获取其所有权（类似于C/Go里的指针？）
            &s                       s1
    -------|--------           -------|--------
    | name | value |           | name | value |
    -------|-------|           -------|--------
    | ptr  |    |   |    |---->| ptr  |       | -----|
                |________|     | len  |   5   |      |
                               | cap  |   5   |      |
                                            |--------|
                        堆内存的内容          |
                    --------|--------       |
                    | index | value |       |
                    |   0   |   h   |<------|
                    |   1   |   e   |
                    |   2   |   l   |
                    |   3   |   l   |
                    |   4   |   0   |
    */
    let s = String::from("test");
    // 将获取引用作为函数参数称为 借用(borrowing)，无法在 calc_length_v2 函数中修改借用的变量，除非借用的变量是可修改的（mut）
    let l = calc_length_v2(&s); // 如果传入的是 s，那么之后s就无效了，但这里传的是s的引用，只使用了其值
    println!("use String after move, s = {}, len = {}", s, l);
    // 修改借用的变量
    let mut s = String::from("Hello");
    edit_borrowed_variable(&mut s);
    println!("after borrowed and edited, s = {}", s); // after borrowed and edited, s = Hello, World!

    /*
    可变引用有一个很大的限制：在特定作用域中的特定数据有且只有一个可变引用，比如以下代码编译会不通过。
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
    报错：cannot borrow `s` as mutable more than once at a time
    */
} // s1 移出作用域并被丢弃

fn edit_borrowed_variable(s: &mut String) {
    s.push_str(", World!");
}

// some_string 作用域开始
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

// some_integer 作用域开始
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域，不会有特俗操作

// gives_ownership 将返回值移动给调用它的函数
fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_string 进入作用域
    some_string // 返回 some_string 并移除给调用它的函数
}

// a_string 进入作用域；takes_and_gives_back 返回传入的字符串
fn takes_and_gives_back(a_string: String) -> String {
    a_string // 返回 a_string 并移出给调用的函数
}

fn calc_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn calc_length_v2(s: &String) -> usize {
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，所以什么也不会发生
