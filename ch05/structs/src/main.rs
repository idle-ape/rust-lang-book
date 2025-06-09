// 定义一个结构体
struct User {
    username: String,
    email: String,
    sign_in_count: i64,
    active: bool,
}

// 定义一个元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // 创建一个结构体实例
    let mut user = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };
    // 通过点号访问结构体字段
    println!("user.username: {}", user.username);
    // 修改结构体实例的值，注意：结构体实例必须要是可修改的，Rust不允许只将某个字段标记为可修改
    user.email = String::from("anotheremail@example.com");

    let u = build_user(String::from("a"), String::from("b"));
    println!(
        "u.sign_in_count = {}, u.active = {}",
        u.sign_in_count, u.active
    );

    /*
    使用结构体更新语法从其他实例创建实例，避免了以下传统的写法
    let new_user = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
        */
    let new_user = User {
        sign_in_count: 1,
        active: false,
        ..user // 剩余未显式设置值的字段与原实例对应字段的值相同
    };
    println!("new_user.active = {}", new_user.active);

    // 元组结构体的初始化（Color和Point是两个不同类型的结构体，即使结构体中的字段有着相同的类型）
    let black = (0, 0, 0);
    let origin = (1, 2, 3);
    // 元组结构体实例类似于元组，可以将其结构为单独的部分，也可以使用.后跟索引来访问单独的值
    let (first, second, third) = black;
    println!(
        "first in black = {}, second in origin  = {}",
        first, origin.1
    );
}

// 通过函数的最后一个表达式来隐式的返回一个结构体实例
fn build_user(email: String, username: String) -> User {
    // User {
    //     email: email,
    //     username: username,
    //     sign_in_count: 1,
    //     active: true,
    // }

    // 参数名与字段名都完全相同，我们可以使用字段初始化简写语法
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}
