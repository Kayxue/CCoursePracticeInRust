use std::io::{self, Write};

fn main() {
    let mut numStr = String::new();
    let mut baseStr = String::new();
    let mut maxBase = 1;
    print!("請輸入一個非負整數：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut numStr).unwrap();
    print!("請輸入進位制：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut baseStr).unwrap();
    let mut num: i32 = numStr.trim().parse().unwrap();
    let base: i32 = baseStr.trim().parse().unwrap();
    maxBase = base;
    while maxBase * base <= num {
        maxBase *= base;
    }
    print!("{}進位表示法：", base);
    while num > 0 {
        print!("{}", num / maxBase);
        num -= (num / maxBase) as i32 * maxBase;
        maxBase /= base;
    }
    print!("\n");
}
