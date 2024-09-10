use std::io;

fn main() {
    println!("欢迎来到猜数字游戏!");

    // 生成一个随机数作为答案
    let secret_number = rand::random::<u32>() % 100 + 1;

    loop {
        println!("请输入你猜测的数字：");

        // 创建一个可变的字符串变量来存储用户的输入
        let mut guess = String::new();

        // 读取用户的输入
        io::stdin().read_line(&mut guess)
            .expect("读取输入失败");

        // 将用户的输入转换为数字类型
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个有效的数字!");
                continue;
            }
        };

        println!("你猜测的数字是: {}", guess);

        // 比较用户的猜测与答案
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("太小了!"),
            std::cmp::Ordering::Greater => println!("太大了!"),
            std::cmp::Ordering::Equal => {
                println!("恭喜你，猜对了!");
                break;
            }
        }
    }
}
