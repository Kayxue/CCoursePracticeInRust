use std::io::{self, Write};

const suitSymbol: [&str; 4] = ["♣", "♦", "♥", "♠"];
const highRank: [&str; 4] = ["J", "Q", "K", "A"];

fn printCard(id: &i8) {
    let symbol = id / 13;
    let num = id % 13;
    if num >= 9 {
        print!(
            "{}{}",
            suitSymbol[symbol as usize],
            highRank[(num - 9) as usize]
        );
    } else {
        print!("{}{}", suitSymbol[symbol as usize], num + 2);
    }
}

fn main() {
    loop {
        let mut inputStr = String::new();
        print!("Input cards: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut inputStr).unwrap();
        let cards: Vec<i8> = inputStr
            .trim()
            .split(" ")
            .map(|e| e.parse().unwrap())
            .collect();
        let mut symbolCount = vec![0; 4];
        let mut numCount = vec![0; 13];
        for k in cards.iter() {
            let symbol = *k / 13;
            let num = *k % 13;
            symbolCount[symbol as usize] += 1;
            numCount[num as usize] += 1;
        }
        for card in cards {
            printCard(&card);
            print!(" ");
        }
        print!("have:\n");
        for (index, value) in symbolCount.iter().enumerate() {
            print!("{}({}) ", suitSymbol[index], value);
        }
        print!("\n");
        for (i, value) in numCount.iter().enumerate() {
            if i >= 9 {
                print!("{}({}) ", highRank[i - 9], value);
            } else {
                print!("{}({}) ", i + 2, value);
            }
        }
        print!("\n\n");
    }
}
