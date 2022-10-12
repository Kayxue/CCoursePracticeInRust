use std::io;
use std::io::Write;

fn power(x:&f32,y:&i32)-> f32{
    if *y==0{
        return 1.0;
    }
    x*power(&x,&(y-1))
}

fn main() {
    let mut repeatTimesStr=String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut repeatTimesStr).unwrap();
    let repeatTime:i32=repeatTimesStr.trim().parse().unwrap();
    for i in 0..repeatTime{
        let mut xStr=String::new();
        let mut nStr=String::new();
        print!("\nx = ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut xStr).unwrap();
        print!("n = ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut nStr).unwrap();
        let (x,n):(f32,i32)=(xStr.trim().parse().unwrap(),nStr.trim().parse().unwrap());
        println!("power({:.6}, {}) = {:.6}\n",&x,&n,power(&x,&n));
    }
}