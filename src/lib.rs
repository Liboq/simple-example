use std::error::Error;
use std::fs;
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let key_content = search(&config.query, &contents);
    println!("Text Content:   \n{} \n query:\n{} \n container:{:?}", contents, config.query,key_content);
    Ok(())
}
pub struct Config {
    pub query: String,
    pub filename: String,
}
pub fn search <'a>(query:&str,content:&'a str)->Vec<&'a str>{
    content.lines().filter(|line| line.contains(&query)).collect()
}
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next();
        let query = match args.next(){
            Some(arg)=>arg,
            None=>return Err("no query")
        };
        let filename = match args.next(){
            Some(arg)=>arg,
            None=>return Err("no filename")
        };
        Ok(Config { query, filename })
    }
}
