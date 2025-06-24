use std::ops::Add;

struct Counter {
    value: u32,
}

pub trait Iterator {
    // 关联类型
    type Item; // Item 是一个类型占位符，trait的实现者会指定Item的具体类型
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.value)
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 在 Point 结构体上实现 Add trait来重载 + 运算符，这样就可以将两个 Point 实例相加了
// Add 默认泛型类型参数是Self，即本身
impl Add for Point {
    type Output = Point; // 指定关联类型的具体类型

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// 实现 Add trait 时自定义 RHS 类型而不是使用默认的类型
#[derive(Debug, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Up");
    }
}

trait Animal {
    fn baby_name() -> String;
}
struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

type Kilometers = i32; // Kilometers 是 i32 的类型别名

type Thunk = Box<dyn Fn() + Send + 'static>;
fn takes_long_type(f: Thunk) {
    // --snip--
}
fn returns_long_type() -> Thunk {
    // --snip--
    Box::new(|| ())
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, // 运算符+重载后，Point 可以进行相加
        Point { x: 3, y: 3 }
    );

    println!("{:?}", Millimeters(1000) + Meters(1));
    assert_eq!(Millimeters(1000) + Meters(1), Millimeters(2000));

    let person = Human;
    // 在方法名前指定 trait 名，显式地指定希望调用哪个 trait 的 fly
    Pilot::fly(&person); // This is your captain speaking.
    Wizard::fly(&person); // Up
    person.fly(); // 等价于 Human::fly(&person), *waving arms furiously*

    // 对于关联函数，因为没有 self 参数，Rust 无法计算出所需的是哪一个 Animal::baby_name 实现
    // Animal::baby_name(); // cannot call associated function on trait without specifying the corresponding `impl` type
    // 需要使用完全限定语法： <Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // A baby dog is called a puppy

    // 引入类型别名来减少代码重复
    let _f: Thunk = Box::new(|| println!("hi"));
}
