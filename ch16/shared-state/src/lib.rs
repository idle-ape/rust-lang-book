/*
互斥器一次只允许一个线程访问数据
*/

use std::{sync::Arc, sync::Mutex, thread};

pub fn use_mutex_in_single_thread() {
    let m = Mutex::new(5);
    {
        // 一旦获取了锁，就可以将返回值(在这里是 num )视为一个其内部数据的可变引用了
        let mut num = m.lock().unwrap();
        *num = 6;
    } // MutexGuard 是一个智能指针，实现了 Deref 来指向其北部数据，也实现了 Drop trait，当 MutexGuard 离开作用域时自动释放锁
    println!("m = {:?}", m);
}

pub fn use_mutex_in_multi_threads() {
    let mutex = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        // 不能将 mutex 锁的所有权移动到多个线程中，需要通过Rc多所有权来修复，但是 Rc<T> 并不能安全的在线程间共享
        // 因为 Rc<T> 并没有使用任何并发原语，来确保改变计数的操作不会被其他线程打断。需要改为原子引用计数 Arc<T> !!!
        let m = Arc::clone(&mutex);
        let handle = thread::spawn(move || {
            let mut num = m.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("Result: {}", mutex.lock().unwrap());
}
