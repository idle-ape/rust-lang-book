struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(width: i32, height: i32) -> Rectangle {
        if width <= 0 || height <= 0 {
            panic!("Weight or Height should gt 0");
        }
        Rectangle {
            width: width,
            height: height,
        }
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

/*
1、#[...]
    这是 Rust 的属性注解（attribute）语法，用于修饰代码的元信息。
2、cfg
    全称 configuration，是 Rust 的条件编译指令，用于根据特定条件决定是否编译某段代码。
3、test
    是 cfg 的一个内置条件，当以下命令运行时，test 条件为真：
    cargo test

#[cfg(test)] 是 rust中的条件编译属性，test表示只有在运行测试时这个模块才会被编译。
类似的还有如：#[cfg(target_os = "linux")]：仅在 Linux 系统下编译。


测试的组织结构：
1、单元测试
    单元测试与他们要测试的代码共同存放在位于 src 目录下相同的文件中。规范是在每个文件中创建包含测试函数的 tests 模块，并使用 cfg(test) 标注模块。
2、集成测试
    创建集成测试，你需要先在项目的根目录创建一个 tests 目录，与 src 同级。接着可以随意在这个目录中创建任意多的测试文件，Cargo 会将每一个文件当作单独的 crate 来编译。
    集成测试对于你需要测试的库来说完全是外部的。同其他使用库的代码一样使用 库文件，也就是说它们只能调用一部分库中的公有 API 。
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test] // #[test]属性表明这是一个测试函数
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        // 通过 assert! 宏来断言 bool 结果，如果值是 true ，assert! 什么也不做，如果值为 false，assert! 调用 panic! 宏
        assert!(
            larger.can_hold(&smaller),
            "smaller rectangle can't hold a larger one!"
        );
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2), "2 + 2 = 4");
        assert_eq!(5, add_two(1), "1 + 2 != 5"); // 自定义的断言错误信息会在断言失败时打印出来
    }

    #[test]
    // 使用 should_panic 属性检查 panic。在函数中的代码 panic 时会通过，而在其中的代码没有 panic 时失败。
    // 为了使 should_panic 测试结果更精确，我们可以给 should_panic 属性增加一个可选的 expected 参数。测试工具会确保错误信息中包含其提供的文本。
    // #[should_panic]
    #[should_panic(expected = "The weight or height should greater than 0")]
    fn width_height_greater_than_0() {
        let rt = Rectangle::new(0, 20);
    }

    // 将 Result<T, E> 用于测试那些返回值是 Result 的函数
    #[test]
    fn test_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
