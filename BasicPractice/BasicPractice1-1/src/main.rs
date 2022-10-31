use std::io::{self, Write};

fn main() {
    let mut dataCountStr = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dataCountStr).unwrap();
    let dataCount: i32 = dataCountStr.trim().parse().unwrap();
    println!(" ");
    for _i in 0..dataCount {
        let mut numStr = String::new();
        print!("Input a number: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut numStr).unwrap();
        let num: i32 = numStr.trim().parse().unwrap();
        if num % 5 == 0 {
            println!("{} is a multiple of 5.", num);
        }
        if num % 7 == 0 {
            println!("{} is a multiple of 7.", num);
        }
        println!(" ");
    }
    println!("Over!");
}
