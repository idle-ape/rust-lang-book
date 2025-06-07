/*
字符串slice
    字符串 slice 是 String 中一部分值的引用，通过 [starting_index..ending_index] 的形式，
    starting_index 是 slice 的第一个位置， ending_index 则是 slice 最后一个位置的后一个值。

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

                s
    | name      | value |       | index | value |
    |-----------|-------|       |-------|-------|
    | ptr       |       | ———-→ | 0     | h     |
    | len       | 11    |       | 1     | e     |
    | capacity  | 11    |       | 2     | l     |
    |           |       |       | 3     | l     |
            world               | 4     | o     |
    | name      | value |       | 5     |       |
    |-----------|-------| ———-→ | 6     | W     |
    | ptr       |       |       | 7     | o     |
    | len       | 5     |       | 8     | r     |
    |           |       |       | 9     | l     |
                                | 10    | d     |
*/
fn main() {
    let s = String::from("hello");
    let slice1 = &s[..2]; // 如果要从第一个索引0开始，可以省略两个点号之前的值，等价于 &s[0..2]
    let slice2 = &s[3..]; // 同理如果是到String的最后一个字节，也可以省略尾部的数组，等价于 &s[3..s.len()]
    let slice3 = &s[..]; // 也可以同时省略两个数字来引用整个字符串
    println!("{}, {}, {}", slice1, slice2, slice3); // he, lo, hello

    let s = String::from("Rust is little difficult to learn.");
    let first = find_first_word(&s);
    println!("The first word of \"{}\" is {}", s, first);

    // 字符串字面值就是 slice，这是一个不可变引用，所以字符串字面值是不可变的
    println!(
        "The first word of \"{}\" is {}",
        "Good Morning!",
        find_first_word("Good Morning!")
    );

    /*
    针对数组的slice，和获取字符串的一部分一样，也可以用slice来引用数组的一部分
    */
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    for (i, &item) in slice.iter().enumerate() {
        println!("i = {}, item = {}", i, item);
    }
}

// fn find_first_world(s: &String) -> &str { // 有经验的会使用下面的函数签名来定义函数，这样既可以传 &String，又可以传 &str
fn find_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
