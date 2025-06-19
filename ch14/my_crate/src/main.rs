/*
库没有将项重导出前的导入：
use my_crate::{kinds::PrimaryColor, utils::mix};
库中通过 pub use 将项重导出后的导入：
use my_crate::PrimaryColor;
use my_crate::mix;
*/
use my_crate::PrimaryColor;
use my_crate::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    println!("{:?}", mix(red, yellow));
}
