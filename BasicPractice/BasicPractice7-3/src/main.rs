use std::io::{self, Write};

fn caculate(a: i32, b: i32, counter: i32, limit: i32) {
    if !(a <= limit && b <= limit) {
        return;
    }
    println!("n = {}, (a, b) = ({}, {})", counter, a, b);
    caculate(a + 2 * b + 1, 3 * a - b + 1, counter + 1, limit);
}

fn main() {
    let mut dataCountStr = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dataCountStr).unwrap();
    let dataCount: i32 = dataCountStr.trim().parse().unwrap();
    for i in 0..dataCount {
        let mut limitStr = String::new();
        print!("\nWhat is the upper limit? ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut limitStr).unwrap();
        let limit: i32 = limitStr.trim().parse().unwrap();
        caculate(1, 1, 1, limit);
    }
}
