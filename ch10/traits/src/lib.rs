/*
trait 类似于其它语言中的接口的功能，虽然有些不同。
trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。
可以通过 trait 以一种抽象的方式定义共享的行为。可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。

Rust 关于实现 trait 的孤儿规则（Orphan Rule）：
    1、你可以为你的类型实现别人的 trait
    例如：你定义了一个 struct Tweet，可以为它实现标准库的 Display trait（因为 Tweet 是你的）。

    2、你也可以为别人的类型实现你的 trait
    例如：你定义了一个 trait Summary，可以为标准库的 Vec<T> 实现它（因为 Summary 是你的）。

    3、但不能为别人的类型实现别人的 trait
    例如：你不能为标准库的 Vec<T> 实现标准库的 Display trait（因为两者都不是你的）。
*/
pub trait Summary {
    // 定义函数列表的同时，也可以在 trait 中有默认实现，这样就不必在每个类型的每个实现中都定义自己的行为
    fn summarize(&self) -> String;
    fn test(&self) -> String {
        format!("Default impl in Summary trait.")
    }
}
