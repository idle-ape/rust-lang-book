/*
一个包含了其他内置了函数的模块的 front_of_house 模块

通过 mod 关键字来声明一个模块，模块内还可以定义其他的子模块；模块还可以保存一些定义的其他项，比如结构体、枚举、常量、特性或者函数

从名为 crate 的模块作为根的 crate 模块结构，称为模块树。
模块树结构：
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的。父模块中的项不能使用子模块中私有的项，反之是可以的。
可以通过 pub 关键字来创建公共项，使子模块的内部部分暴露给上级模块使用
*/

/*
在作用域中增加  use  和路径类似于在文件系统中创建软连接（符号连接，symbolic link）。
通过在 crate 根增加  use crate::front_of_house::hosting ，现在  hosting  在作用域中就是有
效的名称了，如同  hosting  模块被定义于 crate 根一样。通过  use  引入作用域的路径也会
检查私有性，同其它路径一样。
使用 use 将模块引入当前作用域，这样使用 Breakfast 模块下的项时，就不用写冗长的路径前缀
use crate::front_of_house::hosting 和下面的定义等价，这个是绝对路径，下面是相对路径
*/
use front_of_house::Breakfast;

// 使用父模块将两个具有相同名称的类型引入同一作用域
use std::fmt;
/*
使用 as 指定一个新的本地名称，类似于go里的别名
import (
    pkg "github.com/abc/package"
)
*/
use std::io::Result as IoResult;

// 使用 pub 和 use，这种被称为“重导出”，这样做将项引入作用域并同时使其可供其他代码引入自己的作用域
pub use crate::front_of_house::hosting;

/*
通过嵌套路径来消除大量的 use 行，避免引用定义于相同包或相同模块的项时，重复写多个use
use std::cmp::Ordering;
use std::cmp::str;
等价于下面一个 use 的写法

类似于 es6 里的 require?
let { stat, exists, readfile } = require('fs');
*/
use std::{cmp::Ordering, str};

/*
也可以在路径的任何层级使用嵌套路径
以下的写法等价于
use std::io;
use std::io::Write;
*/
use std::io::{self, Write};

/*
如果希望将一个路径下所有的公有项引入作用域，可以在路径后使用 * 这个glob运算符

使用 glob 运算符时请多加小心！Glob 会使得我们难以推导作用域中有什么名称和它们是在何处定义的。
*/
use std::collections::*;

fn function1() -> fmt::Result {
    Ok(())
}

// fn function2() -> io::Result<()> {
fn function2() -> IoResult<()> {
    Ok(())
}

fn server_order() {
    //没有指定 pub use，server_order 函数可以在其作用域中调用 hosting::seat_at_table，但外部代码则不允许使用这个新路径。
    hosting::seat_at_table();
}

mod front_of_house {
    // 也可以使用 pub 来设计公有的结构体和枚举，需要注意的是，即使结构体是公有的，字段不加 pub 的话也是私有的
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruist: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruist: String::from("peaches"),
            }
        }
    }

    // 公有的枚举类型。将枚举设为公有，则它的所有成员都将变为公有
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}
        fn take_payment() {}
    }
    fn fix_incorrect_order() {
        serving::take_order();
        // 通过 super 关键字来构建父模块开始的相对路径
        super::server_order();
    }
}

pub fn eat_at_restaurant() {
    // 通过绝对路径访问模块内的项
    crate::front_of_house::hosting::add_to_waitlist(); // 这里还不能编译，因为 hosting 模块是私有的；hosting公有的还不行，add_to_waitlist()也得是公有的
    // 通过相对路径访问模块内的项
    front_of_house::hosting::add_to_waitlist(); // ditto

    // order a breakfast in the summer with Rye toast
    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    // meal.seasonal_fruist = String::from("duralion"); // seasonal_fruist 字段是私有的，不允许访问
    println!("I'd like {} toast please", meal.toast);

    let order1 = front_of_house::Appetizer::Soup;
    let order2 = front_of_house::Appetizer::Salad;
    println!("order1 = {:?}, order2 = {:?}", order1, order2);
}
