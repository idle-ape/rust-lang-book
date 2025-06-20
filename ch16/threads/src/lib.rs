/*
1、Rust 中使用 spawn 创建新线程
2、线程与 move 闭包
    move 闭包，其经常与 thread::spawn 一起使用，它允许我们在一个线程中使用另一个线程的数据。
*/

use std::{thread, time::Duration};

pub fn print_in_diff_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // 主线程结束后不会等待新线程结束，需要使用 join 等待 handle 所有代表的线程结束
    handle.join().unwrap();
}

pub fn use_move_to_take_the_ownership() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}
