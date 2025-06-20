mod mybox;
mod rc;
mod refcell;
use std::rc::Rc;

use mybox::{List, MyBox};
use rc::RcList::{Cons, Nil};
use refcell::{LimitTracker, Messenger};

fn main() {
    // 使用 box 在堆上储存一个 i32 值
    let b = Box::new(5);
    println!("b = {}", b);

    let l = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("l = {:?}", l);

    let x = 5;
    let y = &x;
    let z = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // 使用解引用符来着追踪 i32 值的引用
    assert_eq!(5, *z); // 在 Box 上使用解引用运算符

    // 自定义智能指针
    let x = 5;
    let y = MyBox::new(x);
    // 在没有实现 Deref 这个 trait 之前，会报错：type `MyBox<{integer}>` cannot be dereferenced
    assert_eq!(5, *y); // 底层运行了如下代码： *(y.deref())

    {
        let mb = MyBox::new(String::from("hello, world"));
    } // mb 从这里离开作用域，所以会打印：Dropping CustomSmartPointer with data: hello, world!

    // std::mem::drop 函数不同于 Drop trait 中的 drop 方法。可以通过传递希望提早强制丢弃的值作为参数。std::mem::drop 位于 prelude，
    // 在值离开作用域之前调用 std::mem::drop 显式清理
    drop(y);
    let mb = MyBox::new(7);

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!(
        "a: {:?}, count after creating a: {}",
        a,
        Rc::strong_count(&a)
    );
    // b 和 c 共享了 a 的所有权
    let b = Cons(3, Rc::clone(&a));
    println!(
        "b: {:?}, count after creating b: {}",
        b,
        Rc::strong_count(&a)
    );

    {
        let c = Cons(4, Rc::clone(&a));
        println!(
            "c: {:?}, count after creating c: {}",
            c,
            Rc::strong_count(&a)
        )
    } // 当 c 离开作用域时，a 的引用计数减 1
    println!("count after c leave his scope: {}", Rc::strong_count(&a));

    let msger = MyMessenger::new();
    let mut lt = LimitTracker::new(&msger, 100);
    lt.set_value(80);
    msger.send("FBI Warning!");
}

struct MyMessenger {}

impl MyMessenger {
    fn new() -> MyMessenger {
        MyMessenger {}
    }
}

impl Messenger for MyMessenger {
    fn send(&self, msg: &str) {
        println!("send a message: {}", msg);
    }
}
