use std::io;
use std::io::Write;

fn main() {
    let mut dataCountStr = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dataCountStr).unwrap();
    let dataCount: i32 = dataCountStr.trim().parse().unwrap();
    for i in 0..dataCount {
        let mut numStr = String::new();
        let mut intervalStr = String::new();
        print!("\n");
        print!("Please input a large integer: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut numStr).unwrap();
        print!("Please input an interval: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut intervalStr).unwrap();
        let mut num: i128 = numStr.trim().parse().unwrap();
        let interval: i32 = intervalStr.trim().parse().unwrap();
        println!("Your input is [{}]", num);
        println!("Its next 10 numbers by adding {} are:", interval);
        for i in 0..10 {
            num += interval as i128;
            println!("{}", num);
        }
        println!("Over!!");
    }
}
