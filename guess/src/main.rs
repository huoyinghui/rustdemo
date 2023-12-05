use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("欢迎来到猜数字游戏!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("请猜测一个介于1到100之间的整数:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("读取输入时出错。");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了!"),
            Ordering::Greater => println!("太大了!"),
            Ordering::Equal => {
                println!("恭喜你，猜中了!");
                break;
            }
        }
    }
}
