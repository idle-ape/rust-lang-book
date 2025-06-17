use minigrep::{self, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    // 使用 Result 下的 unwrap_or_else 方法，优雅的处理出错的情况
    let config = Config::new(&args).unwrap_or_else(|err| {
        // 使用 eprintln! 将错误信息写入标准错误而不是标准输出
        eprintln!("Got problem when parsing argments: {}", err);
        process::exit(1);
    });

    println!(
        "Searching for {} in file {}:",
        config.query, config.filename
    );

    // 不关心 run 成功时的返回值，所以用 if let 来检查是否返回了一个 Err 值，这更合适
    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
