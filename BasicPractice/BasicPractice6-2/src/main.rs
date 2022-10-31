use std::io::{self, Write};

fn main() {
    let mut inputStr = String::new();
    io::stdin().read_line(&mut inputStr).unwrap();
    for c in inputStr.trim().chars() {
        println!(
            "[{}] {} ==> [{}] {}",
            c,
            c as u8,
            (((c as u8) + 1) as char),
            (c as u8) + 1
        )
    }
}
