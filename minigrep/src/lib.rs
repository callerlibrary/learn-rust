use std::error::Error;
use std::fs;

// query 是要搜索的字符串，file_path 是要搜索的文件
pub struct Config {
    query: String,
    file_path: String,
}

// 解释下面的代码：
// 1. Config::build 接受一个字符串切片作为参数，返回一个 Result 类型，其中 Ok 包含一个 Config 实例，Err 包含一个字符串切片。
// 2. 如果 args 的长度小于 3，Config::build 返回一个 Err，其中包含一个字符串切片。
// 3. 如果 args 的长度大于等于 3，Config::build 返回一个 Ok，其中包含一个 Config 实例。
// 4. Config::build 的调用者可以使用 unwrap_or_else 方法来处理 Config::build 返回的 Result。
// 5. 如果 Config::build 返回一个 Err，unwrap_or_else 会调用一个闭包（closure），这个闭包会获取一个参数 err，这个闭包会打印出 err 的值并退出程序。
// 6. 如果 Config::build 返回一个 Ok，unwrap_or_else 会返回 Ok 中的值。
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// 解释下面的代码：
// 定义一个 run 函数，它接受一个 Config 实例作为参数，返回一个 Result 类型，其中 Ok 包含一个 () 实例，Err 包含一个 Box<dyn Error> 实例。
// run 函数的实现体中：
// 1. 使用 fs::read_to_string 函数读取文件并将结果存储在 contents 变量中。
// 2. 使用 println! 宏打印出 contents 的值。
// 3. 使用 search 函数搜索 query 的值并将结果存储在 lines 变量中。
// 4. 使用 Ok 返回一个 () 实例。
// 5. 如果 fs::read_to_string 函数返回一个 Err，run 函数会返回一个 Err，其中包含一个 Box<dyn Error> 实例。
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "Searching for {} In file {}",
        config.query, config.file_path
    );

    let contents = fs::read_to_string(config.file_path)?;

    // println!("With text:\n{contents}");

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

// 解释下面的代码：
// 1. search 函数接受一个字符串切片 query 和一个字符串切片 contents 作为参数，返回一个字符串切片的 vector。
// 2. search 函数的实现体中：
// 3. 创建一个 vector 来存储结果，命名为 results。
// 4. 对 contents 的每一行进行迭代。
// 5. 如果当前行包含 query，将其加入 results。
// 6. 返回 results。
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

// 添加测试
// 创建一个 tests 模块
// 在 tests 模块中创建一个名为 one_result 的函数
// 在 one_result 函数中调用 search 函数并断言其返回值与预期值相等
// 在 main 函数中调用 one_result 函数
// 运行 cargo test
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
}
