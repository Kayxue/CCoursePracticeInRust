use rand::{self, thread_rng, Rng};
use std::io;
use std::io::Write;

fn checkPrize(ticket: &Vec<i32>, firstPrize: &Vec<i32>, special: &i32) -> i32 {
    let mut normalNumCount = 0;
    let mut hasSpecial = false;
    for ticketNum in ticket {
        for firstPrizeNum in firstPrize {
            if ticketNum == firstPrizeNum {
                normalNumCount += 1;
            } else if ticketNum == special {
                hasSpecial = true;
            }
        }
    }
    let matchTruple = (normalNumCount, hasSpecial);
    match matchTruple {
        (6, false) => 8,
        (5, true) => 7,
        (5, false) => 6,
        (4, true) => 5,
        (4, false) => 4,
        (3, true) => 3,
        (2, true) => 2,
        (3, false) => 1,
        others => 0,
    }
}

fn shuffling(arr: &mut Vec<i32>) {
    for i in 0..49 {
        let mut toSwap = i;
        while toSwap == i {
            toSwap = thread_rng().gen_range(0..49);
        }
        let tmp = arr[i];
        arr[i] = arr[toSwap];
        arr[toSwap] = tmp;
    }
}

fn main() {
    let prizeName = [
        "", "General", "Seventh", "Sixth", "Fifth", "Fourth", "Third", "Second", "First",
    ];
    let price = [0, 400, 400, 1000, 1286, 9781, 32705, 1569878, 19100192];
    let mut first: Vec<i32>;
    let mut ticket: Vec<i32>;
    let mut special: i32;

    let mut firstPrizeStr = String::new();
    print!("Input the first-prize numbers: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut firstPrizeStr).unwrap();
    first = firstPrizeStr
        .trim()
        .split(" ")
        .map(|e| e.parse().unwrap())
        .collect::<_>();
    let mut specialNumberStr = String::new();
    print!("Input the special number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut specialNumberStr).unwrap();
    special = specialNumberStr.trim().parse().unwrap();
    let mut ticketStr = String::new();
    print!("Input the numbers on the lottery ticket: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut ticketStr).unwrap();
    ticket = ticketStr
        .trim()
        .split(" ")
        .map(|e| e.parse().unwrap())
        .collect();
    let mut prizeCode = checkPrize(&ticket, &first, &special);
    if prizeCode > 0 {
        println!(
            "You have won the {} Prize which is NT${}!!",
            prizeName[prizeCode as usize], price[prizeCode as usize]
        );
    } else {
        println!("You did not win any prize.");
    }
    println!("\n[Random similation for 5 times]\n");
    for w in 0..5 {
        let mut numArr: Vec<i32> = vec![0; 49];
        for i in 1..=49 {
            numArr[i - 1] = i as i32;
        }
        shuffling(&mut numArr);
        for (i, value) in first.iter_mut().enumerate() {
            *value = numArr[i];
        }
        special = numArr[6];
        shuffling(&mut numArr);
        for (i, value) in ticket.iter_mut().enumerate() {
            *value = numArr[i];
        }
        println!(
            "The first-prize numbers are {}",
            first
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
        println!("The special number is {}", special);
        println!(
            "Your ticket numbers are {}",
            ticket
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
        prizeCode = checkPrize(&ticket, &first, &special);
        if prizeCode > 0 {
            println!(
                "You have won the {} Prize which is NT${}!!",
                prizeName[prizeCode as usize], price[prizeCode as usize]
            );
        } else {
            println!("You did not win any prize.");
        }
        println!(" ");
    }
}
