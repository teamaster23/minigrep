use std::error::Error;
//use std::env; 
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_insensitive: bool, // 是否忽略大小写
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let case_insensitive = if args.len() < 4 { false } else { args[3] == "1" }; // 默认不忽略大小写

        let query = args[1].clone(); // 第一个参数，通常是要搜索的字符串
        let file_path = args[2].clone(); // 第二个参数，通常是要搜索的文件路径
        
        return Ok(Config { query, file_path, case_insensitive });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = if config.case_insensitive {
        //println!("有设置忽略大小写");
        
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    //println!("With text:\n{}", contents);
    for line in results {
        println!("{line}");
    }
    return Ok(());
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    return results;
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    let query = query.to_lowercase(); // 将查询字符串转换为小写

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    return results;
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
        let query = "RUST";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}