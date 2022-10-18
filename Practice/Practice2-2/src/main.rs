use std::io;
use std::io::Write;

fn main() {
    let mut firstPrizeStr = String::new();
    let mut inVoiceNumberStr = String::new();
    let prizeArr = ["頭獎", "二獎", "三獎", "四獎", "五獎", "六獎"];
    print!("請輸入頭獎號碼：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut firstPrizeStr).unwrap();
    print!("請輸入你的發票號碼：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut inVoiceNumberStr).unwrap();
    let firstPrize = firstPrizeStr.trim().chars().rev().collect::<Vec<_>>();
    let invoiceNumber = inVoiceNumberStr.trim().chars().rev().collect::<Vec<_>>();
    let mut correct: i8 = 0;
    for (i, value) in firstPrize.iter().enumerate() {
        if *value == invoiceNumber[i] {
            correct += 1;
        } else {
            break;
        }
    }
    match 8 - correct {
        0..=5 => println!("恭喜！你中了{}！", prizeArr[(8 - correct) as usize]),
        other => println!("抱歉沒有中獎。"),
    }
}
