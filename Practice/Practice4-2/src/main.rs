use std::io;
use std::io::Write;

fn main() {
    let mut xStr = String::new();
    let mut toleranceStr = String::new();
    print!("請輸入 x：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut xStr).unwrap();
    print!("請輸入容許誤差值：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut toleranceStr).unwrap();
    let x: f64 = xStr.trim().parse().unwrap();
    let tolerance = toleranceStr.trim().parse().unwrap();
    let mut result = 0f64;
    let mut now = x;
    let mut count = 1;
    let mut toMultiplyNum = 1;
    while now >= tolerance {
        if count % 2 == 1 {
            result += now;
        } else {
            result -= now;
        }
        count += 1;
        now = (now * x.powi(2)) / (((toMultiplyNum + 1) * (toMultiplyNum + 2)) as f64);
        toMultiplyNum += 2;
    }
    println!("sin({:.6}) ~= {:.20}", x, result);
    println!("sin({:.6}) == {:.20}", x, x.sin());
}
