use std::io;
use std::io::Write;

fn main() {
    let mut dataCountStr = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().expect("Flush Error");
    io::stdin()
        .read_line(&mut dataCountStr)
        .expect("Read Error");
    let dataCount: i32 = dataCountStr.trim().parse().expect("Parse Error");
    println!(" ");
    for _i in 0..dataCount {
        let mut num = String::new();
        print!("Input a number: ");
        io::stdout().flush().expect("Flush Error");
        io::stdin().read_line(&mut num).expect("Read Error");
        print!("Reverse: ");
        for c in num.trim().chars().rev() {
            print!("{}", c);
        }
        io::stdout().flush().expect("Flush Error");
        println!("\n");
    }
}
