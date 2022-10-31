use std::io::{self, Write};

fn main() {
    let mut count = 0;
    loop {
        let mut answerStr = String::new();
        print!("請猜一個被 3 除餘 2、被 5 除餘 3、被 7 除餘 2 的數：");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut answerStr).unwrap();
        let answer: i32 = answerStr.trim().parse().unwrap();
        count += 1;
        if answer % 3 == 2 && answer % 5 == 3 && answer % 7 == 2 {
            println!("猜對了！");
            break;
        } else {
            println!("錯！");
        }
    }
    match count {
        1 => println!("哇！一次就猜中！超強！"),
        2..=5 => println!("猜了 {} 次才猜對，有待加強。", count),
        other => println!("猜了 {} 次才猜對，沒人比你強。", count),
    }
}
