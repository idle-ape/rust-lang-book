/*
模式有两种形式
1、refutable（可反驳的）：对某些可能的值进行匹配会失败的模式被称为是可反驳的
    if let Some(x) = a_value 表达式中的Some(x) ;如果变量 a_value 中的值是 None 而不是 Some ，那么 Some(x) 模式不能匹配
2、inrefutable（不可反驳的）：能匹配任何传递的可能值的模式
    一个例子就是 let x = 5; 语句中的 x ， 因为 x 可以匹配任何值所以不可能会失败
*/

struct Point {
    x: i32,
    y: i32,
}

struct MyStruct {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    ChangeNewColor(Color),
}

fn main() {
    // 所有的模式语法
    // 1、匹配字面值
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 2、匹配命名变量
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // 它会匹配任何 Some 中的值
        _ => println!("Default case, x = {:?}", x),
    }
    println!("At the end: x = {:?}, y = {:?}", x, y);

    // 3、多个模式
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 4、通过 ..= 匹配值的范围，范围只允许用于数字或 char 值
    let x = 5;
    match x {
        1..5 => println!("one through five"),
        _ => println!("something else"),
    }

    // 5、解构结构体
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p; // 创建了变量 a 和 b 来匹配结构体 p 中的 x 和 y 字段
    assert_eq!(a, 0);
    assert_eq!(b, 7);
    // 因为变量名匹配字段名是常见的，同时因为 let Point { x: x, y: y } = p; 包含了很多重复，
    // 所以对于匹配结构体字段的模式存在简写：只需列出结构体字段的名称，则模式创建的变量会有相同的名称。
    let Point { x, y } = p;
    assert_eq!(x, 0);
    assert_eq!(y, 7);
    // 也可以使用字面值作为结构体模式的一部分进行进行解构，而不是为所有的字段创建变量
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // 6、解构枚举
    let message = Message::ChangeColor(0, 160, 255);
    // 解构包含不同类型值成员的枚举
    match message {
        // 没有任何数据的枚举成员，不能进一步解构，只能匹配其字面值
        Message::Quit => println!("The Quit variant has no data to destructure"),
        // 类结构体枚举成员，可以采用类似于匹配结构体的模式
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        // 这种包含一个或多个元素的类元组枚举成员，类似于解构元组
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, blue {}", r, g, b);
        }
        // 匹配嵌套的枚举
        Message::ChangeNewColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, blue {}", r, g, b);
        }
        Message::ChangeNewColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change color to hue {}, saturation {}, and value {}",
                h, s, v
            );
        }
    }

    // 7、混合解构结构体和元组
    let ((_feet, _inches), Point { x, y }) = ((3, 10), Point { x: 3, y: 10 });

    // 8、忽略模式中的值
    foo(4, 4);

    // 9、使用嵌套的 _ 忽略部分值
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting value: {:?}", setting_value);
    // 忽略元组的多个部分
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    // 10、使用 .. 忽略剩余值，.. 会扩展为所需要的值的数量
    let origin = MyStruct { x: 0, y: 0, z: 0 };
    match origin {
        MyStruct { x, .. } => println!("x is {}", x),
    }
    let nums = (1, 3, 9, 27, 81);
    match nums {
        (first, .., last) => println!("Some numbers: {}, {}", first, last),
    }

    // 11、匹配守卫提供的额外条件
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // 12、@绑定，at 运算符( @ )允许我们在创建一个存放值的变量的同时测试其值是否匹配模式
    enum HelloWorld {
        Hello { id: i32 },
    }
    let hw = HelloWorld::Hello { id: 5 };
    match hw {
        HelloWorld::Hello { id: id_var @ 3..7 } => println!("Found an id in range: {}", id_var),
        HelloWorld::Hello { id: 10..12 } => println!("Found an id in another range"), // 只在模式中指定了一个范围，分支相关代码代码没有一个包含 id 字段实际值的 变量
        HelloWorld::Hello { id } => println!("Found some other id: {}", id),
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
