use std::io::{self, Write};

fn main() {
    let mut dataCountStr = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dataCountStr).unwrap();
    let dataCount: i32 = dataCountStr.trim().parse().unwrap();
    for _i in 0..dataCount {
        let mut aStr = String::new();
        let mut digitsStr = String::new();
        let mut a: i32 = -1;
        let mut digits: i32 = -1;
        while a <= 0 || a >= 10 {
            print!("Input the value of a (1 ~ 9): ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut aStr).unwrap();
            a = aStr.trim().parse().unwrap();
            aStr = String::new();
        }
        while digits <= 0 || digits >= 10 {
            print!("Input the number of digits (1 ~ 9): ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut digitsStr).unwrap();
            digits = digitsStr.trim().parse().unwrap();
            digitsStr = String::new();
        }
        print!("s = ");
        let mut nowa = a;
        let mut sum = 0;
        for _i in 0..digits - 1 {
            print!("{} + ", nowa);
            sum += nowa;
            nowa = (nowa * 10) + a;
        }
        print!("{} = {}\n\n", nowa, sum + nowa);
    }
}
