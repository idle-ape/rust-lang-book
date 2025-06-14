/*
需要先在 Cargo.toml 的 dependencies 是添加 traits 依赖
如果这两个 crate 属于同一个项目，可以考虑使用 Cargo workspace 来管理，这样它们可以共享依赖和编译缓存。具体配置方式：

在项目根目录创建 Cargo.toml：

[workspace]
members = ["traits", "trait_impl"]

将 traits 和 trait_impl 移动到工作空间目录下。
这样，trait_impl 可以直接依赖 traits，而无需指定相对路径。
*/
use std::{
    fmt::{Debug, Display, Formatter, Result},
    iter::Sum,
};
use traits::Summary;

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 为类型 NewArticle 实现 trait（类似于 Go 里面实现某个 interface）
impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    // 重载 trait 的 test 方法
    fn test(&self) -> String {
        format!("impl trait func test in NewArticle")
    }
}

impl Display for NewArticle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.author, self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 为类型 Tweet 实现 trait（类似于 Go 里面实现某个 interface）
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    // 这里不重载test方法，调用 trait 默认的
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_book"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}, test: {}", tweet.summarize(), tweet.test());

    let article = NewArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
        hockey team in the NHL.",
        ),
    };
    println!(
        "New article available! {}, test: {}",
        article.summarize(),
        article.test()
    );

    notify(&article);
    notify_with_trait_bound(tweet);
    notify_with_multi_trait_bound(article);

    println!(
        "summary of returned summarizable: {}",
        return_summarizable().summarize()
    )
}

/*
trait 作为函数参数，任何实现了 Summary 这个 trait 的都可以传递进来

impl trait 语法适用于直观的例子，它是 trait bound 的语法糖，适用于这种短小的例子
*/
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/*
traint bound 语法，trait bound 与泛型参数声明在一起，位于尖括号中的冒号后面
*/
fn notify_with_trait_bound<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

/*
如果需要的入参得实现多个 trait，可以通过 + 号指定多个 trait bound
*/
// fn notify_with_multi_trait_bound(item: impl Summary + Display) {
fn notify_with_multi_trait_bound<T: Summary + Display>(item: T) {
    println!(
        "Notify with multi trait bound. {}, {}",
        item.summarize(),
        item
    );
}

/*
如果每个泛型都有自己的 trait bound，所以有多个泛型参数的函数在名称和参数列表之间会有很长的 trait bound 信息，可以通过 where 从句简化 trait bound，
这个函数签名就先得不那么杂乱
*/
fn notify_use_multi_trait_with_where<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

/*
函数返回一个实现了某个 trait 的类型：
在 Rust 中，impl Trait 作为返回类型时，函数必须在所有代码路径上返回同一种具体类型，即使这些类型都实现了相同的 trait（如 Summary）。
Tweet 和 NewsArticle 是不同的类型，因此编译器会报错，解决方案：
1、使用 Box<dyn Trait>（动态分发）
    let switch = false;
    if switch {
        Box::new(Tweet {
            username: "Hello, world".to_string(),
            content: "of course...".to_string(),
            reply: false,
            retweet: false,
        })
    } else {
        Box::new(NewsArticle {
            headline: String::from("Penguins win!"),
            // ...其他字段
        })
    }
2、使用枚举（Enum）包装不同类型
    enum SummaryItem {
        Tweet(Tweet),
        NewsArticle(NewsArticle),
    }
    impl Summary for SummaryItem {
        fn summarize(&self) -> String {
            match self {
                SummaryItem::Tweet(t) => t.summarize(),
                SummaryItem::NewsArticle(a) => a.summarize(),
            }
        }
    }
*/
fn return_summarizable() -> impl Summary {
    // let switch = false;
    // if switch {
    Tweet {
        username: "Hello, world".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    }
    // } else {
    //     NewArticle {
    //         headline: String::from("Penguins win the Stanley Cup Championship!"),
    //         location: String::from("Pittsburgh, PA, USA"),
    //         author: String::from("Iceburgh"),
    //         content: String::from(
    //             "The Pittsburgh Penguins once again are the best
    //         hockey team in the NHL.",
    //         ),
    //     }
    // }
}
