use std::io::{self, Write};

fn main() {
    let mut str = String::new();
    print!("Input a line  : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut str).unwrap();
    print!("Encrypted line: ");
    for c in str.chars() {
        match c {
            'A'..='Z' => print!(
                "{}",
                (((((c as u8) - 'A' as u8) + 2) % 26) + 'A' as u8) as char
            ),
            'a'..='z' => print!(
                "{}",
                (((((c as u8) - 'a' as u8) + 2) % 26) + 'a' as u8) as char
            ),
            '0'..='9' => print!(
                "{}",
                (((((c as u8) - '0' as u8) + 9) % 10) + '0' as u8) as char
            ),
            other => print!("{}", c),
        }
    }
    io::stdout().flush().unwrap();
}
