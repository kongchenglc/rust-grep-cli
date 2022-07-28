use minigrep::Config;
use std::env;
use std::process;

// main 函数一般只用来调用lib.rs。 lib.rs来放业务逻辑
// 1.参数
// 2.其他配置
// 3.调用lib.rs
// 4.处理可能出现的错误
fn main() {
    // 接收命令行参数
    // let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parse args: {}", err); //标准错误，不会输出到标准输出文件 //标准输出:cargo run > output.txt
        process::exit(1);
    });

    // if let 就是match的语法糖
    if let Err(e) = minigrep::run(config) {
        eprintln!("app err {}", e);
        process::exit(1);
    }
}
