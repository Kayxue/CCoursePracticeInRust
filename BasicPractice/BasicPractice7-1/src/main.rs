use std::io::{self, Write};

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn main() {
    let mut dataCountStr = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dataCountStr);
    let dataCount: i32 = dataCountStr.trim().parse().unwrap();
    for i in 0..dataCount {
        let mut aStr = String::new();
        let mut bStr = String::new();
        print!("\na = ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut aStr).unwrap();
        print!("b = ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut bStr).unwrap();
        let a: i32 = aStr.trim().parse().unwrap();
        let b: i32 = bStr.trim().parse().unwrap();
        let result = gcd(a, b);
        println!("GCD({}, {}) = {}", a, b, result);
    }
}
