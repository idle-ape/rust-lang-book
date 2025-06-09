// 通过 enum 关键字来定义枚举类型
#[derive(Debug)]
enum IPAddKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IPAddr {
    kind: IPAddKind,
    address: String,
}

// 每个枚举都关联了 String 值
#[derive(Debug)]
enum IPVersion {
    // V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

/*
枚举可以关联到不同类型的值

结构体也可以包含相同的数据，如下：
struct QuitMessage; // 类单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体
*/
enum Message {
    Quit,                       // 没有关联任何类型
    Move { x: i32, y: i32 },    // 关联了一个匿名结构体
    Write(String),              // 关联了一个 String 类型
    ChangeColor(i32, i32, i32), // 关联到3个i32
}

// 枚举和结构体相似，也可以使用 impl 来为枚举定义方法
impl Message {
    fn call(&self) {
        println!("call function of enum");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 绑定值模式
}

/*
Option 枚举模式，它只包含有值和没值两种情况，它定义于标准库中
enum Option<T> {
    Some(T),
    None,
}

Option<T> 枚举是如此有用以至于它甚至被包含在了 prelude 之中，你不需要将其显式引入作用域。
*/

fn main() {
    // 使用枚举值
    let v6 = IPAddKind::V6;
    // 将 IP 地址的数据和 IpAddrKind 成员存储在一个 struct 中
    let loopback = IPAddr {
        kind: IPAddKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("loopback = {:?}", loopback);

    // 直接将数据附加到枚举的每个成员上，这样就不需要一个额外的结构体了
    let home = IPVersion::V4(127, 0, 0, 1);
    let loopback = IPVersion::V6(String::from("::1"));
    println!("loopback = {:?}, home  = {:?}", loopback, home);

    // 创建一个枚举类型的变量
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    // 如果使用 None 而不是 Some ，需要告诉 Rust Option<T> 是什么类型的，因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型
    let absent_number: Option<i32> = None;
    println!(
        "some_number = {:?}, origin some_number = {}, some_string = {:?}, absent_number = {:?}",
        some_number,
        some_number.unwrap(), // 从 Some 成员中取出 T 的值来使用它，Option<T> 枚举拥有大量用于各种情况的方法
        some_string,
        absent_number
    );

    let value_in_cents = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{:?} eq {} cents.", Coin::Penny, value_in_cents);

    println!("1 plus_one = {:?}", plus_one(Some(1)));

    // 匹配是穷尽的，正常情况下 match 的所有分支需要处理到所有可能的情况，但是 Rust 也提供了一个模式用于不想列举出所有可能值的场景，即 _ 通配符
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // 通配符会匹配所有的值，将其放置于其他分支之后，将会匹配所有之前没有置顶的可能值
    }

    // match 模式匹配对于在只关心一种情况的场景中有点啰嗦，因此 Rust 提供了 if let
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

// 使用match模式表达式来处理枚举的控制流
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // 变量 state 将会绑定 25 美分硬币所对应的州的值
        Coin::Quarter(state) => {
            println!("State qurter from {:?}!", state); // State qurter from Alaska!
            25
        }
    }
}

// 通过 match 匹配 Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
