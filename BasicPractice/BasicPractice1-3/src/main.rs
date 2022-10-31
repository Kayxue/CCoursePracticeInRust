use std::io::{self, Write};

fn main() {
    let mut numStr = String::new();
    print!("Please input a positive number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut numStr).unwrap();
    let num: i32 = numStr.trim().parse().unwrap();
    for i in 1..=num {
        println!(
            "{} / {} = {:0.6}",
            i,
            i + 1,
            ((i as f32) / ((i + 1) as f32))
        );
    }
}
