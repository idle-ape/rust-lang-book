use std::vec;

/*
迭代器都实现了一个叫做 Iterator 的定义于标准库的 trait:
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // 此处省略了方法的默认实现
}

通过 iter 方法创建一个迭代器，如果我们需要一个获取所有权并返回拥有所有权的迭代器，则可以调用 into_iter 而不是 iter。
类似的，如果我们希望迭代可变引用，则可以调用 iter_mut 而不是 iter。

也可以用标准库中其他的集合类型创建迭代器，比如哈希 map。另外，可以实现 Iterator trait 来创建任何我们希望的迭代器。
*/
fn main() {
    let v = vec![1, 2, 4];
    // 迭代器需要是可变的，因为调用 next 方法改变了迭代器中用来记录序列位置的状态
    // 使用 for 循环时无需使 v_iter 可变因为 for 循环会获取 v_iter 的所有权并在后台使 v_iter 可变。
    let mut v_iter = v.iter();
    // 在迭代器上直接调用 next 方法
    assert_eq!(v_iter.next(), Some(&1));
    assert_eq!(v_iter.next(), Some(&2));

    // 消费迭代器的方法：调用 next 方法的方法被称为消费适配器(consuming adaptors)，因为调用他们会消耗迭代器。
    // 一个消费适配器的例子是 sum 方法。这个方法获取迭代器的所有权并反复调用 next 来遍历迭代器，因而会消费迭代器。
    let total: i32 = v_iter.sum();
    assert_eq!(total, 4);

    // 产生其他迭代器的方法：Iterator trait 中定义了另一类方法，被称为迭代器适配器(iterator adaptors)，他们允许我们将当前迭代器变为不同类型的迭代器。
    // 可以链式调用多个迭代器适配器。不过因为所有 的迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果。
    let v = vec![1, 2, 3];
    let v: Vec<i32> = v.iter().map(|elem| elem + 1).collect();
    for i in v {
        println!("{}", i);
    }

    // 自定义迭代器
    for i in Counter::new().into_iter() {
        println!("{}", i);
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// 迭代器使用闭包获取环境
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        assert_eq!(
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                }
            ],
            shoes_in_my_size(shoes, 10)
        )
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(sum, 18);
    }
}

// 实现 Iterator trait 来创建自定义迭代器
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // 将迭代器的关联类型 Item 设置为 u32 ，意味着迭代器会返回 u32 值集合
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
