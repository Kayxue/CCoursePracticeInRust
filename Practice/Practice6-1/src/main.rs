use std::io::{self, Write};

const suitSymbol: [&str; 4] = ["♣", "♦", "♥", "♠"];
const highRank: [&str; 4] = ["J", "Q", "K", "A"];

fn printCard(id: &i8) {
    let symbol = id / 13;
    let num = id % 13;
    if num >= 9 {
        println!(
            "{}{}",
            suitSymbol[symbol as usize],
            highRank[(num - 9) as usize]
        );
    } else {
        println!("{}{}", suitSymbol[symbol as usize], num + 2);
    }
}

fn main() {
    loop {
        let mut cardNumberStr = String::new();
        print!("Input a card ID: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut cardNumberStr).unwrap();
        let cardNumber: i8 = cardNumberStr.trim().parse().unwrap();
        if cardNumber == -1 {
            break;
        }
        printCard(&cardNumber);
    }
}
