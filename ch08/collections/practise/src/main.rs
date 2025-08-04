use std::{collections::HashMap, io};

fn main() {
    let mut v = vec![2, 1, 3, 7,4, 5, 12, 3, 2];
    let mut sum = 0;
    for ele in &v {
        sum += ele;
    }
    // 平均数
    println!("avg: {}", sum/v.len());

    v.sort();
    // 中位数
    println!("after sort: {:?}, middle num: {:?}", v, v.get(v.len()/2));

    // 每个数字出现的次数
    let mut m = HashMap::new();
    for ele in &v {
        let count =  m.entry(ele).or_insert(0);
        *count +=1;
    }
    println!("{:?}", m);

    /*
    使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。
    接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。
    */
    let mut m = HashMap::new();
    let input = io::stdin();
    for line in input.lines() {
        if let Ok(l) = line {
            if l.trim().to_lowercase() == "quit" {
                break;
            }

            let words: Vec<String> = l.split_whitespace().map(|s| s.to_string()).collect();
            if let (Some(name), Some(department)) = (words.get(1), words.get(3)) {
                m.entry(department.clone()).or_insert_with(Vec::new).push(name.clone()); // 如果部门不存在，则创建一个空的 vector
            }
        }
    }
    println!("{:?}", m);

    for (key, value) in &m {
        println!("{}: {:?}", key, value);
    }
}
