use std::vec;

/*
Rust 中广泛使用的集合：
不同于内建的数组和元组类型，这些集合指向的数据是储存在堆上的，这意味着数据的数量不必在编译时就已知，并且还可以随着程序的运行增长或缩小。

1、vector
    允许我们一个挨着一个的储存一系列数量可变的值，但是只能储存相同类型的值
2、字符串 String
3、哈希map：允许我们将值与一个特定的键想关联
*/
fn main() {
    // 创建一个空的 vector，这里需要使用类型注解，因为 vector 是使用范型实现的，创建空的 vector时没有插入任何值，Rust不知道我们要存储什么类型
    let v: Vec<i32> = Vec::new();
    // 使用初始值来创建一个 vector，这时候就不需要类型注解，因为Rust可以推断出想要存放的类型
    let mut v = vec![1u8, 2u8, 3u8];
    // 使用 push 方法向 vector 增加值
    v.push(5u8);
    v.push(6u8);
    // 丢弃 vector时也会丢弃其所有元素
    {
        let vec = vec![1, 2, 3, 4];
        // 处理变量vec
    } // 这里 vec 离开作用域并被丢弃

    // 读取 vector 中的元素的两种方式
    let third: &u8 = &v[2]; // 通过索引读取，所以是从 0 开始的；当索引超过时即访问一个不存在的元素时会造成panic
    println!("The third element is {}", third);
    // 通过 get 方法读取
    match v.get(3) {
        Some(elem) => println!("The fourth element is {}", elem),
        None => println!("There is no fourth elment"),
    }
    /*
    不能在相同作用域中同时存在可变和不可变引用
    error[E0502]: cannot borrow `ve` as mutable because it is also borrowed as immutable
    --> src/main.rs:37:5
    |
    36 |     let first = &ve[0];
    |                  -- immutable borrow occurs here
    37 |     ve.push(6);
    |     ^^^^^^^^^^ mutable borrow occurs here
    38 |     println!("The first element is {}", first);
    |                                         ----- immutable borrow later used here
    */
    let mut ve = vec![1, 2, 3, 4, 5];
    let first = &ve[0];
    // ve.push(6); // 当打开这行的注释时，编译运行会报错，因为同时存在了不可变引用和可变引用
    println!("The first element is {}", first);

    // 通过 for 循环遍历 vector 中的元素
    for i in &ve {
        println!("{}", i);
    }

    // 遍历一个可变的 vector 中的元素并修改其中的元素
    let mut mutable_vec = vec![2, 4, 6, 8];
    for i in &mut mutable_vec {
        *i = *i * 2;
    }

    // 使用枚举来储存多种类型，以便能在 vector 中存放不同类型的数据
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.5),
        SpreadsheetCell::Text(String::from("bule")),
    ];
}
