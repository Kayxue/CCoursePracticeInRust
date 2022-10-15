use rand::{self, Rng};
use std::io;
use std::io::Write;
const hiddenDigits: usize = 4;

fn seperateDigits(numStr: &str) -> Vec<i8> {
    numStr.chars().map(|e| (e as i8) - 48).collect::<Vec<_>>()
}

fn hiddenNumberGenerator() -> Vec<i8> {
    let num = rand::thread_rng().gen_range(1000..=9999).to_string();
    seperateDigits(&num)
}

fn checkA(hidden: &Vec<i8>, guess: &Vec<i8>) -> i8 {
    let mut sum = 0;
    for (index, value) in hidden.iter().enumerate() {
        if *value == guess[index] {
            sum += 1;
        }
    }
    sum
}

fn checkB(hidden: &Vec<i8>, guess: &Vec<i8>) -> i8 {
    let mut sum = 0;
    for (indexHidden, valueHidden) in hidden.iter().enumerate() {
        for (indexGuess, valueGuess) in guess.iter().enumerate() {
            if valueHidden == valueGuess && indexGuess != indexHidden {
                sum += 1;
            }
        }
    }
    sum
}

fn hasDuplicateDigit(guess: &Vec<i8>) -> bool {
    let mut count: [i8; 10] = [0; 10];
    for i in guess {
        if count[*i as usize] == 1 {
            return true;
        } else {
            count[*i as usize] = 1;
        }
    }
    false
}

fn ifNumberis4Digit(num: &i32) -> bool {
    (num / 10000) as i16 == 0
}

fn main() {
    /*--------------Basic--------------*/
    println!("[Basic]");
    let mut hiddenNumberBasic: Vec<i8>;
    let mut guessBasic: Vec<i8>;
    hiddenNumberBasic = seperateDigits(&(9527.to_string()));
    guessBasic = seperateDigits(&(7523.to_string()));
    println!(
        "{}A{}B",
        checkA(&hiddenNumberBasic, &guessBasic),
        checkB(&hiddenNumberBasic, &guessBasic)
    );
    /*--------------Advanced--------------*/
    println!("[Advanced]");
    let mut hiddenNumber: Vec<i8> = hiddenNumberGenerator();
    let mut guessNumber: Vec<i8> = vec![0; hiddenDigits];
    let mut count = 0;
    loop {
        let mut guessStr = String::new();
        print!("請猜一個 4 位數不重覆的數字：");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut guessStr).unwrap();
        let guessNum: i32 = guessStr.trim().parse().unwrap();
        if guessNum <= 0 {
            println!("只能猜正整數。");
            continue;
        }
        if !ifNumberis4Digit(&guessNum) {
            println!("只能猜 4 位數的數字。");
            continue;
        }
        guessNumber = seperateDigits(&(guessStr.trim()));
        if hasDuplicateDigit(&guessNumber) {
            println!("不能有重覆數字。");
            continue;
        }
        count += 1;
        let A = checkA(&hiddenNumber, &guessNumber);
        let B = checkB(&hiddenNumber, &guessNumber);
        println!("{}A{}B", A, B);
        if A == hiddenDigits as i8 {
            break;
        }
    }
    println!("猜對了！你猜了 {} 次。", count)
}
