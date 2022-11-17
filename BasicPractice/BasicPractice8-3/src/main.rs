use std::io::{self, Write};

unsafe fn pay_amount(
    dollars: &mut i32,
    fifties: *mut i32,
    tens: *mut i32,
    fives: *mut i32,
    ones: *mut i32,
) {
    *fifties = *dollars / 50;
    *dollars %= 50;
    *tens = *dollars / 10;
    *dollars %= 10;
    *fives = *dollars / 5;
    *dollars %= 5;
    *ones = *dollars;
}

fn main() {
    let mut dataCountStr = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dataCountStr).unwrap();
    let dataCount: i32 = dataCountStr.trim().parse().unwrap();
    for i in 0..dataCount {
        let mut inputStr = String::new();
        print!("\nPlease enter the price: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut inputStr).unwrap();
        let mut dollar: i32 = inputStr.trim().parse().unwrap();
        let mut num50 = 0;
        let mut num10 = 0;
        let mut num5 = 0;
        let mut num1 = 0;
        unsafe {
            pay_amount(&mut dollar, &mut num50, &mut num10, &mut num5, &mut num1);
        }
        println!(
            "You need {} coin(s), including:",
            num50 + num10 + num5 + num1
        );
        println!("\t$50: {}", num50);
        println!("\t$10: {}", num10);
        println!("\t$5: {}", num5);
        println!("\t$1: {}", num1);
    }
}
