use std::io::{self, Write};

fn main() {
    let mut dataCountStr = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dataCountStr).unwrap();
    let dataCount: i32 = dataCountStr.trim().parse().unwrap();
    for i in 0..dataCount {
        let mut inputString = String::new();
        let mut skippingStr = String::new();
        print!("\nInput a string: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut inputString).unwrap();
        print!("Skipping = ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut skippingStr).unwrap();
        let skipping: usize = skippingStr.trim().parse().unwrap();
        inputString = inputString.trim().to_string();
        println!("[{}]", inputString);
        while inputString.len() > skipping {
            inputString = inputString[skipping..].to_string();
            println!("[{}]", inputString);
        }
    }
}
