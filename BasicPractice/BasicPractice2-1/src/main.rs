use std::io;
use std::io::Write;

fn main() {
    let mut dataCountStr: String = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().expect("Flushed Falled");
    io::stdin()
        .read_line(&mut dataCountStr)
        .expect("readline Error");
    let dataCount: i32 = dataCountStr.trim().parse().expect("Parse Error");
    println!(" ");
    for i in 0..dataCount {
        let mut firstStr = String::new();
        let mut limitStr = String::new();
        let mut differenceStr = String::new();
        print!("The first number: ");
        io::stdout().flush().expect("Flushed Falled");
        io::stdin().read_line(&mut firstStr).expect("read Failed");
        print!("The upper limit: ");
        io::stdout().flush().expect("Flushed Falled");
        io::stdin().read_line(&mut limitStr).expect("read Failed");
        print!("Common difference: ");
        io::stdout().flush().expect("Flushed Falled");
        io::stdin()
            .read_line(&mut differenceStr)
            .expect("read Failed");
        let first: i32 = firstStr.trim().parse().expect("Parse Failed");
        let limit: i32 = limitStr.trim().parse().expect("Parse Failed");
        let difference: usize = differenceStr.trim().parse().expect("Parse Failed");
        let mut sum = 0;
        for i in (first..=limit).step_by(difference) {
            sum += i;
        }
        println!("The sum of this arithmetic sequence is {} \n",sum);
    }
}
