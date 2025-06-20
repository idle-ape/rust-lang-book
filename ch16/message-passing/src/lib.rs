/*
Rust 中一个实现消息传递并发的主要工具是通道(channel)。这个思想来源于 Go 编程语言文档中 的口号：
“不要共享内存来通讯;而是要通讯来共享内存。”(“Do not communicate by sharing memory; instead, share memory by communicating.”)
*/

use std::{sync::mpsc, thread, time::Duration}; // Multi-producer, single-consumer

pub fn communicate_between_thread_with_channel() {
    // tx 代表发送者，rx代表接收者；当发送者或接收者任一被丢弃时可以认为通道被 关闭(closed)了。
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let val = String::from("hi");
        // val 通过通道发送之后，send函数获取其参数的所有权并移动这个值归接收者所有
        if let Err(err) = tx.send(val) {
            // println!("send msg {} to tx err: {}", val, err); // 这里不能使用 val 了
            println!("send msg to tx err: {}", err);
        }
    });

    // 通过 recv 来接收消息，recv会阻塞线程，直到有消息到来
    // 与之对应的是 try_recv，调用 try_recv 会立即返回一个 Result<T, E>: Ok 值包含可用的信息，
    // 而 Err 值代表此时没有任何消息，并不会因为通道里没消息而阻塞线程
    let received = rx
        .recv()
        .unwrap_or_else(|err| String::from(format!("receive msg err: {}", err)));
    println!("receive msg: {}", received);
    handle.join().unwrap();
}

pub fn recv_msg_with_iterator() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 而是将 rx 当作一个迭代器，当通道被关闭时，迭代器也将结束
    // Go 中也有同样的用法
    for msg in rx {
        println!("Got: {}", msg);
    }
}

// 克隆通道的发送端来实现多个发送者
pub fn clone_tx_to_send_with_multi_producer() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); // 等价于 mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // rust 标准库中不支持多消费者，如果要实现多消费者，可以使用社区广泛使用的增强版通道的 crossbeam-channel
    for msg in rx {
        println!("Got: {}", msg);
    }
}
