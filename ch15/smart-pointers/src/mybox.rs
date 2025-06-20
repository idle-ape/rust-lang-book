/*
box 智能指针，其类型是 Box<T>，允许将一个值放在堆上而不是栈上，多用于如下场景：
    1、当有一个在编译时位置大小的类型，而又想要在去要确切大小的上下文中使用这个类型值的时候
    2、当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
    3、当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
*/

use std::fmt::Display;
use std::ops::Deref;

pub struct MyBox<T: Display>(T);

impl<T> MyBox<T>
where
    T: Display,
{
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>
where
    T: Display,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/*
指定在值离开作用域时应该执行的代码的方式是实现 Drop trait。
可以为任何类型提供 Drop trait 的实现，同时所指定的代码被用于释放类似于文件或网络连接的资源。
*/
impl<T> Drop for MyBox<T>
where
    T: Display,
{
    // Drop trait 的 drop 方法不允许手动调用，它会在变量离开作用域时自动调用
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}!", self.0);
    }
}

/*
enum List {
    Cons(i32, List),
    Nil,
}
尝试定义一个递归枚举时，会得到如下错误：
recursive type `List` has infinite sizerustcClick for full compiler diagnostic
main.rs(15, 15): recursive without indirection
main.rs(15, 15): insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle: `Box<`, `>`
*/
// 为了拥有已知大小而使用 Box<T> 的 List 定义
#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
