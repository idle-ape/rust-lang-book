/*
在类型定义中使用泛型，在结构体名称后使用<>声明泛型参数的类型，类比 Go 中结构体使用泛型：
type Point[T, U any] struct {
    x T
    y U
}
*/
struct Point<T, U> {
    x: T,
    y: U,
}

// 在使用了泛型的结构体上实现方法时，使用泛型，通过在 impl 后面使用<>声明泛型类型
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    // 方法使用了与结构体定义中不同类型的泛型
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

/*
Rust 中可以只针对某种具体的类型实现某个方法，而不是针对泛型，比如如下代码，只针对 T 和 U 是 f32 才有 distance_from_origin，
其它类型则没有定义此方法
*/
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

enum Option<T> {
    Some(T), // Some 的枚举值和类型 T 绑定
    None(),
}
enum Result<T, E> {
    Ok(T),  // Ok 的枚举值和类型 T 绑定
    Err(E), // Err 的枚举值和类型 E 绑定
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    assert_eq!(result, 100);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 'b', 'f', 'y'];
    let result = largest_char(&char_list);
    assert_eq!(result, 'y');
    println!("The largest char is {}", result);

    // 在函数中使用泛型
    let float_list = vec![1.2, 3.4, 5.6, 7.8];
    let result = largest_generic(&float_list);
    assert_eq!(*result, 7.8);
    println!("The largest float64 is {}", result);

    // 在结构体中使用泛型
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    let dist = Point {
        x: 2.5f32,
        y: 3.7f32,
    };
    let diff_type_in_struct_and_fn = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "hello", y: 'c' };
    println!(
        "both_integer.x = {}, both_float.x = {}, integer_and_float.x = {}, dist = {}, diff_type_in_struct_and_fn.y = {}",
        both_integer.x(),
        both_float.x(),
        integer_and_float.x(),
        dist.distance_from_origin(),
        // integer_and_float.distance_from_origin(), // no method named `distance_from_origin` found for struct `Point<{integer}, {float}>` in the current scope
        diff_type_in_struct_and_fn.mixup(p2).y,
    );
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &num in list {
        if num > largest {
            largest = num;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &num in list {
        if num > largest {
            largest = num;
        }
    }
    largest
}

/*
在函数中使用泛型，泛型版本的最大值函数，类比 Go 中的泛型：
func largest[T constraints.Ordered](slice []T) T {
    num := slice[0]
    for _, v := range slice {
        if v > num {
            num = v
        }
    }
    return num
}

Rust 和 Go 中泛型参数的类型定义都位于函数名后面，不同的是 Rust 中用<>，而Go中用的是[]
*/
fn largest_generic<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for num in list {
        if num > largest {
            largest = num;
        }
    }
    largest
}
