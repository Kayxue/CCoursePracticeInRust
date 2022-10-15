use std::io;

fn main() {
    let mut numStr = String::new();
    io::stdin().read_line(&mut numStr).unwrap();
    let num: i32 = numStr.trim().parse().unwrap();
    for i in 1..=num {
        println!("{} ^ 3 = {}", i, i.pow(3));
    }
}
