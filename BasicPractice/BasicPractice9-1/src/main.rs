use std::io::{self, Write};

fn main() {
    let mut dataCountStr = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dataCountStr).unwrap();
    let dataCount: i8 = dataCountStr.trim().parse().unwrap();
    for i in 0..dataCount {
        let mut inputStr = String::new();
        println!("\nInput a line:");
        io::stdin().read_line(&mut inputStr).unwrap();
        inputStr = inputStr.trim().to_string();
        println!("Modified line:");
        for ch in inputStr.chars() {
            if !((ch >= '0' && ch <= '9') || ch == '%' || ch == '.') {
                print!("_");
            } else {
                print!("{}", ch);
            }
        }
        print!("\n");
    }
}
