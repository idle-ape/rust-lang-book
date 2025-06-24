/*
有四类可以在不安全 Rust 中进行而不能用于安全 Rust 的操作
1、解引用裸指针
    和引用一样，裸指针是可变或不可变的，分别写作 *mut T 和 *const T。这里的星号不是解引用运算符，它是类型名称的一部分。
    在裸指针的上下文中，不可变意味着指针解引用之后不能直接赋值。
2、调用不安全的函数或方法
3、访问或修可变静态变量
4、实现不安全 trait
5、访问 union 的字段
*/

// 使用 extern 函数调用外部代码， extern 有助于创建和使用外部函数接口(Foreign Function Interface，FFI)
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "hello, world";

fn main() {
    let mut num = 5;
    // 通过引用创建裸指针。可以在安全代码中创建裸指针，不能在不安全块之外解引用裸指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 在不安全的代码块中解引用裸指针
    unsafe {
        println!("r1 is : {}", *r1);
        println!("r2 is : {}", *r2);
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(3));
    }
    // 定义和使用一个不可变静态变量
    println!("name is: {}", HELLO_WORLD);
}

#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
