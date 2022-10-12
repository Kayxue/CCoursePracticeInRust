use std::io;
use std::io::Write;

fn main() {
    let mut numStr = String::new();
    print!("Please input a positive number: ");
    io::stdout().flush().expect("Flush Failed");
    io::stdin().read_line(&mut numStr).expect("Read Failed");
    let num: i32 = numStr.trim().parse().expect("Parse Error");
    for i in 1..=num {
        println!(
            "{} / {} = {:0.6}",
            i,
            i + 1,
            ((i as f32) / ((i + 1) as f32))
        );
    }
}
