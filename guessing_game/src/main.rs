use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub struct Guess {
    value: i32
}

impl Cuess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("你这个数字不能小于1或者大于100{}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

}
fn main() {
    println!("猜数字");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("请填入你的数字");


        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("读不到你的数字");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        Guess::new(guess);

        println!("你的数字是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("恭喜你猜中了！");
                break;
            },
        }
    }

}
