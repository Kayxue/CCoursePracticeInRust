use std::io;
use std::io::Write;

fn main() {
    let mut dataCountStr = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().expect("Flushed Failed");
    io::stdin()
        .read_line(&mut dataCountStr)
        .expect("Read Failed");
    let dataCount: i32 = dataCountStr
        .trim()
        .parse()
        .expect("DataCount Parsed Failed");
    println!(" ");
    for _i in 0..dataCount {
        let mut numStr = String::new();
        print!("Input a number: ");
        io::stdout().flush().expect("Flushed Failed");
        io::stdin().read_line(&mut numStr).expect("Read Failed");
        let num: i32 = numStr.trim().parse().expect("num parsed failed");
        if num % 5 == 0 {
            println!("{} is a multiple of 5.", num);
        }
        if num % 7 == 0 {
            println!("{} is a multiple of 7.", num);
        }
        println!(" ");
    }
    println!("Over!");
}
