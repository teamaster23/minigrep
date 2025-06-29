use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone(); // 第一个参数，通常是要搜索的字符串
        let file_path = args[2].clone(); // 第二个参数，通常是要搜索的文件路径
        
        return Ok(Config { query, file_path});
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    println!("With text:\n{}", contents);
    return Ok(());
}
