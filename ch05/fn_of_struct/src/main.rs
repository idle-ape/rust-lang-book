#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
使用 impl 块使方法定义于 Rectangle 上下文中。
允许在 impl 块中定义不以 self 作为参数的函数，这种函数被称为关联函数，比如 String::from()
关联函数经常被用作返回一个结构体新实例的构造函数。

每个结构体都允许拥有多个 impl 块，比如可以把 Rectangle 的方案都放到单独的 impl 块：
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
*/
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    /*
    在 C/C++ 语言中，有两个不同的运算符来调用方法: . 直接在对象上调用方法，而 -> 在一个对象的指针上调用方法，这时需要先解引用(dereference)指针。
    换句话说， 如果 object 是一个指针，那么 object->something() 就像 (*object).something() 一 样。

    Rust 并没有一个与 -> 等效的运算符；相反，Rust 有一个叫自动引用和解引用 (automatic referencing and dereferencing)的功能。
    方法调用是 Rust 中少数几个拥有这种行为的地方。
    他是这样工作的：当使用 object.something() 调用方法时，Rust 会自动为 object 添加 & 、&mut 或 * 以便使 object 与方法签名匹配。
    */
    let area = rect.area(); // 等价于 (&rect).area()
    println!("The area of the rectangle is {} square pixles.", area);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 通过关联函数来初始化一个结构体实例，:: 语法用于关联函数和模块创建的命名空间
    let sq = Rectangle::square(3);
    println!("sq area is {}", sq.area())
}
