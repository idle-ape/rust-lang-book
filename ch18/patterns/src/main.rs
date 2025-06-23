//! 可能会用到模式的位置

use std::vec;

fn main() {
    // 组合并匹配 if let、else if 和 else if let
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("Useing your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Useing purple as the background color");
        } else {
            println!("Useing orange as the background color");
        }
    } else {
        println!("Useing blue as the background color");
    }

    // while let 条件循环
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // 使用 for 循环来解构
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // 使用模式解构元组并一次创建三个变量
    let (x, y, z) = (1, 2.98, "hello");

    // 在参数中使用模式的函数签名
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("current location: ({}, {})", x, y);
}
