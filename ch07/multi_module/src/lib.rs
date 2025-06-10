// 在 mod front_of_house 后使用分号，而不是代码块 {}，表示在另一个与模块同名的文件中夹在模块的内容
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
