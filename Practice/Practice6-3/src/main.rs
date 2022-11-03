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

fn shuffling(numArr: &mut Vec<i32>) {
    for i in 0..numArr.len() {
        let mut toSwap = i;
        while toSwap == i {
            toSwap = thread_rng().gen_range(0..numArr.len());
        }
        let temp = numArr[toSwap];
        numArr[toSwap] = numArr[i];
        numArr[i] = temp;
    }
}

fn checkFlush(suitCount: &Vec<i32>) -> bool {
    suitCount.contains(&5)
}

fn checkStraight(rankCount: &Vec<i32>) -> bool {
    let mut count = 0;
    for i in (*rankCount).iter().rev() {
        if *i != 0 {
            count += 1;
        } else {
            count = 0;
        }
        if count == 5 {
            return true;
        }
    }
    false
}

fn main() {
    let mut generated = 0;
    let mut firstInput = true;
    loop {
        let mut cards = vec![0; 5];
        if generated < 10 {
            let mut nums = vec![0i32; 52];
            for (index, value) in nums.iter_mut().enumerate() {
                *value = index as i32;
            }
            shuffling(&mut nums);
            for i in 0..5 {
                cards[i] = nums[i];
            }
        } else {
            if firstInput {
                print!("\n");
                firstInput = false;
            }
            let mut inputStr = String::new();
            print!("請輸入 5 張牌：");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut inputStr).unwrap();
            cards = inputStr
                .trim()
                .split(" ")
                .map(|e| e.parse().unwrap())
                .collect();
        }
        let mut suitCount = vec![0i32; 4];
        let mut rankCount = vec![0i32; 14];
        let mut countSlot = vec![0i32; 5];
        for card in cards {
            printCard(&(card as i8));
            print!(" ");
            let symbol = card / 13;
            let num = card % 13;
            rankCount[(num + 1) as usize] += 1;
            suitCount[symbol as usize] += 1;
        }
        for i in &rankCount {
            countSlot[(*i) as usize] += 1;
        }
        rankCount[0] = rankCount[13];
        let isFlush = checkFlush(&suitCount);
        let isStraight = checkStraight(&rankCount);
        let checkTuple = (isFlush, isStraight);
        match checkTuple {
            (true, true) => print!("同花順"),
            (true, false) => print!("同花"),
            (false, true) => print!("順"),
            (flush, straight) => {
                if countSlot[4] == 1 {
                    print!("四條");
                } else if countSlot[3] == 1 && countSlot[2] == 1 {
                    print!("葫蘆");
                } else if countSlot[3] == 1 {
                    print!("三條");
                } else if countSlot[2] == 2 {
                    print!("兩對");
                } else if countSlot[2] == 1 {
                    print!("一對");
                } else {
                    print!("無");
                }
            }
        }
        print!("\n");
        io::stdout().flush().unwrap();
        generated += 1;
    }
}
