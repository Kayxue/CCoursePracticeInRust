use std::io;
use std::io::Write;

fn isPrime(num:&i32) -> bool{
    for i in 2..=((*num as f32).sqrt() as i32){
        if num%i==0{
            return false;
        }
    }
    true
}

fn main() {
    let mut dataCountStr = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dataCountStr).unwrap();
    let dataCount: i32 = dataCountStr.trim().parse().unwrap();
    for t in 0..dataCount {
        let mut numStr=String::new();
        print!("\nInput a number (>= 6): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut numStr).unwrap();
        let num:i32=numStr.trim().parse().unwrap();
        for i in 2..=num{
            for j in i..=num{
                let k=num-i-j;
                if isPrime(&i) && isPrime(&j) && isPrime(&k) && i<=j && j<=k && k>0{
                    println!("{} = {} + {} + {}",num,i,j,k);
                }
            }
        }
    }
}
