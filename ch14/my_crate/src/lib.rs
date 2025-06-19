// //! 用来生成库文档，来对整个库进行一个说明
// 用 /// 来编写文档注释

//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain calculations more convenient.

/// pub use 语句重导出项
/// 这样调用方就不必知道库的内部组织结构，直接 模块名::PrimaryColor 的方式直接使用即可
pub use crate::kinds::PrimaryColor;
pub use crate::kinds::SecondaryColor;
pub use crate::utils::mix;

pub mod kinds {
    /// 采用 RGB 色彩模式的主要颜色。
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// 采用 RGB 色彩模式的次要颜色。
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;
    /// 等量的混合两个主要颜色来创建一个次要颜色。
    ///
    /// # Examples
    ///
    /// ```
    /// let arg = 5;
    /// let anwser = my_crate::add_one(arg);
    ///
    /// assert_eq!(6, anwser);
    /// ```
    pub fn mix(_c1: PrimaryColor, _c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        SecondaryColor::Orange
    }
}
