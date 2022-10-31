use rand::{self, thread_rng, Rng};
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

fn shuffling(arr: &mut Vec<usize>) {
    for k in 0..arr.len() {
        let mut toSwap = k;
        while toSwap == k {
            toSwap = thread_rng().gen_range(0..arr.len());
        }
        let temp = arr[k];
        arr[k] = arr[toSwap];
        arr[toSwap] = temp;
    }
}

fn main() {
    let mut nums = vec![0; 52];
    for (index, value) in nums.iter_mut().enumerate() {
        *value = index;
    }
    shuffling(&mut nums);
    let mut firstPrint = true;
    for (index, value) in nums.iter().enumerate() {
        if index % 13 == 0 {
            if !firstPrint {
                print!("\n");
            }
            print!("Player {}:", (index / 13) + 1);
            firstPrint = false;
        }
        print!(" ");
        printCard(&(*value as i8));
    }
    print!("\n\n");
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
        print!("\n");
    }
}
