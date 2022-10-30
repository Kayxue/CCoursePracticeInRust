use std::io::Write;
use std::{io, vec};

fn main() {
    let mut nStr = String::new();
    let mut kStr = String::new();
    print!("n = ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nStr).unwrap();
    print!("k = ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut kStr).unwrap();
    let n: usize = nStr.trim().parse().unwrap();
    let k: usize = kStr.trim().parse().unwrap();
    let mut data = vec![0; n];
    let mut nowLeave = 1;
    let mut nowCount = 0;
    let mut canFill = true;
    while nowLeave <= n {
        for e in data.iter_mut() {
            if nowLeave > n {
                break;
            }
            if *e == 0 {
                canFill = true;
                nowCount += 1;
            }
            if nowCount % k == 0 && canFill {
                *e = nowLeave;
                nowLeave += 1;
                canFill = false;
            }
        }
    }
    for (index, value) in data.iter().enumerate() {
        println!("The leaving order of person#{} is {}", index + 1, *value);
    }
}
