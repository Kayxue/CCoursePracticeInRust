use std::io;

fn main() {
    let mut numStr=String::new();
    io::stdin().read_line(&mut numStr).expect("Reading Failed");
    let num:i32=numStr.trim().parse().expect("Parse num Failed");
    for i in 1..=num{
        println!("{} ^ 3 = {}",i,i.pow(3));
    }
}
