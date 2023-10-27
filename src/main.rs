use std::env;
use std::process;
use guess_game::Config;
fn main(){
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        println!("problem parsing argments:{}",err);
        process::exit(1);
    });
    if let Err(e) = guess_game::run(config){
        println!("Application error : {}",e);
        process::exit(1)
    }
   
}