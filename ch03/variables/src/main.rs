fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    // 如果变量x前面没有加mut，则表示x是不可修改的
    // 对一个不可修改的变量二次赋值编译时会报错：cannot assign twice to immutable variable
    x = 6;
    println!("The value of x is: {}", x);
}
