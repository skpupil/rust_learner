use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("请猜测一个数");

    loop {
        let secret_num = rand::thread_rng().gen_range(1, 5);

        let mut guess = String::new(); //::关联函数
        io::stdin().read_line(&mut guess).expect("无法读取");
        println!("你输入的是：{}", guess);
        println!("神秘数字：{}", secret_num);
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("equal");
                break;
            }
        }
    }
}
