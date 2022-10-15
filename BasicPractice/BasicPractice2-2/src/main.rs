use std::io;
use std::io::Write;

fn main() {
    let mut dataCountStr = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dataCountStr).unwrap();
    let dataCount: i32 = dataCountStr.trim().parse().unwrap();
    println!(" ");
    for _i in 0..dataCount {
        let mut num = String::new();
        print!("Input a number: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut num).unwrap();
        print!("Reverse: ");
        for c in num.trim().chars().rev() {
            print!("{}", c);
        }
        io::stdout().flush().unwrap();
        println!("\n");
    }
}
