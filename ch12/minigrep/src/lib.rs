use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    case_insensitive: bool,
}

impl Config {
    // 通过关联函数构造结构体实例
    pub fn new(args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let mut params = args.skip(1);
        let query = match params.next() {
            Some(query) => query,
            None => return Err("Didn't get a query string"),
        };
        let filename = match params.next() {
            Some(filename) => filename,
            None => return Err("Didn't get filename"),
        };
        Ok(Config {
            query,
            filename,
            case_insensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

// Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，不过无需指定具体将会返回的值的类型。
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let result: Vec<&str>;
    if config.case_insensitive {
        result = search(config.query.as_str(), contents.as_str());
    } else {
        result = search_case_insensitive(config.query.as_str(), contents.as_str());
    }
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut result: Vec<&str> = Vec::new();
    for line in lines {
        let l = line.trim();
        if l.contains(query) {
            result.push(l);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result = Vec::new();
    // for line in contents.lines() {
    //     if line.to_uppercase().contains(query.to_uppercase().as_str()) {
    //         result.push(line);
    //     }
    // }
    // result
    contents
        .lines()
        .filter(|line| line.to_uppercase().contains(query.to_uppercase().as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "DUCT";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(true, {
            let mut result: bool = false;
            for r in search_case_insensitive(query, contents) {
                if r.contains("safe, fast, productive.") {
                    result = true;
                }
            }
            result
        });
    }
}
