use std::io;
use std::io::Write;

fn main() {
    let mut numStr = String::new();
    print!("Please input a positive number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut numStr).unwrap();
    let mut num: i32 = numStr.trim().parse().unwrap();
    let mut firstPrint = true;
    print!("{} = ", num);
    for i in 2..=((num as f32).sqrt() as i32) {
        let mut sum = 0;
        while num % i == 0 {
            sum += 1;
            num /= i;
        }
        if sum != 0 {
            if firstPrint {
                firstPrint = false;
            } else {
                print!(" * ");
            }
            print!("{}^{}", i, sum);
        }
    }
    if num != 1 {
        if !firstPrint {
            print!(" * ");
        }
        print!("{}^1", num);
    }
    print!("\n");
}
