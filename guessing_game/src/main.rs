use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("猜数字!");

    loop {
        println!("请输入你的猜测.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读取失败");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的是: {guess}");
        // println!("谜底是: {secret_number}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小了!"),
            Ordering::Greater => println!("大了!"),
            Ordering::Equal => {
                println!("对了!");
                break;
            }
        }
    }
}
