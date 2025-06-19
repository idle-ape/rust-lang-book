/// 1、Crate 名称转换：
///     在 Cargo.toml 中，crate 名称通常使用 kebab-case（短横线分隔，如 add-one），但在 Rust 代码中，
///     需要使用 snake_case（下划线分隔，如 add_one）来引用这个 crate
///     这是因为 Rust 的标识符命名规范要求模块、函数等名称使用 snake_case
/// 2、为什么不需要 use：
///     虽然你可以使用 use add_one; 来显式导入，但这不是必须的
///     Rust 会自动将依赖项作为一个模块引入到当前 crate 的根作用域中
///     你可以直接通过 crate 名称（转换后的）来访问其中的公共项
/// 3、工作原理：
///     当你声明一个依赖项时（在 Cargo.toml 中），Rust 会自动使该 crate 在代码中可用
///     编译器会自动处理 crate 名称的转换（从 add-one 到 add_one）

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}, plus two is {}!",
        num,
        add_one::add_one(num),
        add_two::add_two(num)
    );
}
