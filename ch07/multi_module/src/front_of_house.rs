/*
Rust 会按照以下顺序查找 hosting 模块：
    front_of_house/hosting.rs（推荐方式）
    front_of_house/hosting/mod.rs（旧风格，仍然支持但逐渐淘汰）
如果 hosting.rs 不放在 front_of_house 目录下，Rust 会报错，因为它找不到该模块。
*/
pub mod hosting;
