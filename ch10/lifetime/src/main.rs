/*
生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的。

当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。
如果返回的引用没有指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值，它将会是一个悬垂引用，因为它将会在函数结束时离开作用域。
在这种情况，最好的解决方案是返回一个有所有权的数据类型而不是一个引用，这样函数调用者就需要负责清理这个值了。

每一个引用都有一个生命周期，而且我们需要为那些使用了引用的函数、方法或结构体指定生命周期。


编译器采用三条规则来判断引用何时不需要明确的注解：
1、每一个是引用的参数都有它自己的生命周期参数。
    即有一个引用参数的函数有一个生命周期参数: fn foo<'a>(x: &'a i32) ，有两个引用参数的函数有两个不同的生命周期参数， fn foo<'a, 'b>(x: &'a i32, y: &'b i32) ，依此类推。
2、如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
    fn foo<'a>(x: &'a i32) -> &'a i32
3、如果方法有多个输入生命周期参数，不过其中之一因为方法的缘故为 &self 或 &mut self ，那么 self 的生命周期被赋给所有输出生命周期参数。
*/

use std::fmt::Display;

// 定义一个存放引用的结构体，所以其定义需要生命周期注解。这个注解意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存 在的更久。
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    // 适用于规则3，返回值类型被赋予了 &self 的生命周期
    fn announce_and_return_part(&self, annoucement: &str) -> &str {
        println!("Attention please: {}", annoucement);
        self.part
    }
}

impl<'a> Display for ImportantExcerpt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.part)
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);

    let r = longest_with_an_announcement(string1.as_str(), string2, i);
    println!("{}", r);
}

/*
如果用以下的函数签名是没办法编译通过的：
fn longest(str1: &str, str2: &str) -> &str { // missing lifetime specifier
因为Rust并不知道将要返回的引用是指向 str1 还是 str2，应该采用生命周期注解语法。

生命周期注解并不改变任何引用的生命周期的长短。与当函数签名中指定了泛型类型参数后，就可以接受任何类型一样，
当指定了泛型生命周期后函数也能接受任何生命周期的引用。生命周期注解描述了多个引用生命周期相互的关系，而不影响其生命周期。

比如以下函数定义指定了签名中所有的引用必须有相同的生命周期 'a。被 'a 所替代的具体生命周期是 x 的作用域与 y 的作用域相重叠的那一部分。
换一种说法就是泛型生命周期 'a 的具体生命周期等同于 x 和y 的生命周期中较小的那一个。
因为我们用相同的生命周期参数 'a 标注了返回的引用值，所以返回的引用值就能保证在 x 和 y 中较短的那个生命周期结束之前保持有效。
*/
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() { str1 } else { str2 }
}

// 结合泛型类型参数、trait bounds 和生命周期
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}
