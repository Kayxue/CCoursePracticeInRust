use std::io::{self, Write};

fn main() {
    let mut dataCountStr: String = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dataCountStr).unwrap();
    let dataCount: i32 = dataCountStr.trim().parse().unwrap();
    println!(" ");
    for i in 0..dataCount {
        let mut firstStr = String::new();
        let mut limitStr = String::new();
        let mut differenceStr = String::new();
        print!("The first number: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut firstStr).unwrap();
        print!("The upper limit: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut limitStr).unwrap();
        print!("Common difference: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut differenceStr).unwrap();
        let first: i32 = firstStr.trim().parse().unwrap();
        let limit: i32 = limitStr.trim().parse().unwrap();
        let difference: usize = differenceStr.trim().parse().unwrap();
        let mut sum = 0;
        for i in (first..=limit).step_by(difference) {
            sum += i;
        }
        println!("The sum of this arithmetic sequence is {} \n", sum);
    }
}
