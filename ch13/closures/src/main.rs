use std::{collections::HashMap, thread, time::Duration, vec};

// 定义一个结构体用于缓存闭包计算的结果，这样闭包只需要调用一次
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calc: T,
    value: HashMap<u32, u32>, // 用map存对应key的结果
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(func: T) -> Cacher<T> {
        Cacher {
            calc: func,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if let Some(n) = self.value.get(&arg) {
            return *n;
        }
        let n = (self.calc)(arg);
        self.value.entry(arg).or_insert(n);
        n
    }
}

fn main() {
    let itensity = 7u32;
    let random_number = 5u32;
    // generate_workout_without_closure(itensity, random_number);
    // generate_workout_with_closure(itensity, random_number);
    generate_workout_with_closure_and_cacher(itensity, random_number);

    /*
    闭包会捕获其环境。闭包可以通过三种方式捕获其环境，他们直接对应函数的三种获取参数的方式：获取所有权，可变借用和不可变借用。
    这三种捕获值的方式被编码为如下三个 Fn trait:
    1、FnOnce: 消费从周围作用域捕获的变量，闭包周围的作用域被称为其环境，environment。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。
    如果你希望强制闭包获取其使用的环境值的所有权，可以在参数列表前使用 move 关键字。 这个技巧在将闭包传递给新线程以便将数据移动到新线程中时最为实用。
    其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
    2、FnMut: 获取可变的借用值所以可以改变其环境
    3、Fn: 从其环境获取不可变的借用值
    */
    let x = 4;
    // 一个引用了其周围作用域中变量的闭包
    let equal_to_x = |num| num == x;
    let y = 3;
    assert!(equal_to_x(x));
    assert!(equal_to_x(y));

    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x); // borrow of moved value: `x`; value borrowed here after move
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

fn simulated_expensive_calculation(num: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
}

/*
不使用闭包的版本，解决了第一个 if 块中不必要的两次调用函数的问题。
但现在所有的情况下都需要调用函数并等待结果，包括那个完全不需要这一结果的内部 if 块。
*/
fn generate_workout_without_closure(itensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(itensity);
    if itensity < 25 {
        println!("Today, do {} pushups", expensive_result);
        println!("Next, do {} situps", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay Hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}

/*
在程序的一个位置指定某些代码，并只在程序的某处实际需要结果的时候执行这些代码，这正是闭包的用武之地!

闭包的定义以一对竖线( | )开 始，在竖线中指定闭包的参数，如果有多个参数，可以用逗号分隔，如：|param1, param2|
参数之后是存放闭包体的大括号，如果闭包体只有一行则大括号是可以省略的。
*/
fn generate_workout_with_closure(itensity: u32, random_number: u32) {
    /*
    闭包不要求像 fn 函数那样在参数和返回值上注明类型。即参数 num 和返回值的类型注解不像函数那样是必须的，
    但是如果尝试对同一闭包使用不同类型则会得到类型错误。

    闭包通常很短并只与对应相对任意的场景较小的上下文中。在这些有限制的上下文中，编译器能可靠的推断参数和返回值的类型，
    类似于它是如何能够推断大部分变量的类型一样。
    */
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if itensity < 25 {
        println!("Today, do {} pushups", expensive_closure(itensity));
        println!("Next, do {} situps", expensive_closure(itensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay Hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(itensity));
        }
    }
}

// 使用闭包且带缓存的版本，这样就避免闭包执行多次的情况
fn generate_workout_with_closure_and_cacher(itensity: u32, random_number: u32) {
    let mut expensive_cacher = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if itensity < 25 {
        println!("Today, do {} pushups", expensive_cacher.value(itensity));
        println!("Next, do {} situps", expensive_cacher.value(itensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay Hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_cacher.value(itensity)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn get_value() {
        let mut cacher = Cacher::new(|num| num);
        let _val1 = cacher.value(1);
        let val2 = cacher.value(2);
        assert_eq!(val2, 2);
    }
}
