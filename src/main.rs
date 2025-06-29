use std::env;
use std::process; // 用于处理进程相关的功能，如退出程序

use minigrep::Config; // 引入 Config 结构体，用于解析命令行参数

fn main(){
    let args: Vec<String> = env::args().collect();//包含程序启动时的所有命令行参数，并返回一个字符串向量
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    }); // 解析命令行参数，获取查询字符串和文件路径

    //dbg!(args);//打印变量的值和位置信息

    println!("Searching for '{}' in file '{}'", config.query, config.file_path);

    if let Err(e) = minigrep::run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    }; // 调用 run 函数来处理文件内容
}
