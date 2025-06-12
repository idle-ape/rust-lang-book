use std::collections::HashMap; // HashMap没有被 prelude 自动引用，所以需要通过 use 引入当前作用域

/*
哈希 map 将它们的数据储存在堆上。类似于 vector，哈希 map 是同质的，即所有的键必须是相同类型，值也必须都是相同类型。
*/

fn main() {
    // 创建一个空的 HashMap
    let mut scores = HashMap::new();
    // 插入一些键值对
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 另一种创建 map 的方法
    // 其中 Orange 与 30 是一对，依此类推
    let teams = vec![String::from("Orange"), String::from("Purple")];
    let initial_scores = vec![30, 70];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效
    // 对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者

    // 访问map中的值
    let team = String::from("Orange");
    let score = scores.get(&team);
    println!("team {} score = {:?}", team, score);

    // map 的遍历，会以任意顺序打印出每一个键值对
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    // 覆盖一个值
    let mut m = HashMap::new();
    m.insert(String::from("a"), 1u16);
    m.insert(String::from("a"), 3u16); // a 这个键对应的值已经被覆盖为 3 了

    // 只在键没有对应值时插入
    m.entry(String::from("b")).or_insert(6);
    m.entry(String::from("b")).or_insert(5);
    println!("{:?}", m); // {"b": 6, "a": 3}，b的值没有被覆盖

    // 根据旧值更新一个值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // or_insert() 返回的是这个键的对应值的一个可变引用( &mut V )
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map); // {"world": 2, "wonderful": 1, "hello": 1}
}
