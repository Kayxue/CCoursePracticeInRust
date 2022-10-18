use std::io;
use std::io::Write;

fn main() {
    let mut aStr = String::new();
    let mut bStr = String::new();
    print!("Product A 購買數量：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut aStr).unwrap();
    print!("Product B 購買數量：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut bStr).unwrap();
    let mut a: i32 = aStr.trim().parse().unwrap();
    let mut b: i32 = bStr.trim().parse().unwrap();
    let mut sum = 0;
    if a > b {
        sum += 39 * b;
        a -= b;
        b = 0;
        sum += (a / 2) * 40;
        if a % 2 != 0 {
            sum += 25;
        }
    } else {
        sum += 39 * a;
        b -= a;
        a = 0;
        sum += (b / 2) * 32;
        if b % 2 != 0 {
            sum += 20;
        }
    }
    println!("總共是 {} 元。", sum);
}
