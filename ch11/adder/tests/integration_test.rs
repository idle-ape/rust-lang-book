/*
集成测试：
    每一个 tests 目录中的文件都被编译为单独的 crate，tests 目录中的文件不能像 src 中的文件那样共享相同的行为。
    tests 文件夹在 Cargo 中是一个特殊的文件夹， Cargo 只会在运行 cargo test 时编译这个目录中的文件。

    cargo test --test integration_test
    可以通过在 --test 后面跟文件名来运行 tests 目录下指定的集成测试。

集成测试中的子模块：
    tests 目录下的文件都会当作集成测试。当把一些集成测试的帮助函数移动到其它文件时（比如 tests/common.rs），运行集成测试的结果中也会看到对应文件的测试结果部分，
    即便这个文件没有包含任何测试函数。
    为了避免这种情况，我们要创建的是 tests/common/mod.rs，而不是 tests/common.rs，则是一种命名规范，告诉 rust 不要将 common 看作一个集成测试文件

如果项目是二进制 crate 并且只包含 src/main.rs 而没有 src/lib.rs，这样就不可能在 tests 目录创建集成测试并使用 extern crate 导入 src/main.rs 中定义的函数。
只有库 crate 才会向其 他 crate 暴露了可供调用和使用的函数;二进制 crate 只意在单独运行。

为什么 Rust 二进制项目的结构明确采用 src/main.rs 调用 src/lib.rs 中的逻辑的方式？
因为通过这种结构，集成测试就可以通过 extern crate 测试库 crate 中的主要功能了，而如果这些重要的功能没有问题的话，src/main.rs 中的少量代码也就会正常工作且不需要测试。
*/
use adder;
mod common; // 在另一个与模块同名的文件中加载模块的内容

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
