#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // 增加注解来派生 Debug trait，并使用调试格式打印 Rectangle 实例
    println!("rect is {:#?}", rect);
    println!(
        "The area of the rectangle is {} square pixles.",
        area(&rect)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
