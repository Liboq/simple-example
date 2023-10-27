use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("猜数游戏！！");
    let secret_num = rand::thread_rng().gen_range(1..101);
    loop {
        println!("猜测一个数");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜测的数是：{}", guess);
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("神秘数字是:{}", secret_num);
                println!("you win ");
                break;
            }
        }
    }
}
