use std::io::{self, Write};

const PI: f64 = 3.14159265358979323846;

fn gcd(a1: &i32, b1: &i32) -> i32 {
    let mut a = *a1;
    let mut b = *b1;
    let mut t: i32;
    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() {
    let mut nStr = String::new();
    print!("n = ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nStr).unwrap();
    let n: i32 = nStr.trim().parse().unwrap();
    for i in 0..=2 * n {
        if i % n == 0 {
            println!(
                "x = {} PI, sin(x) = {:.6}, cos(x) = {:.6}",
                i / n,
                (i as f64 * PI / n as f64).sin(),
                (i as f64 * PI / n as f64).cos()
            );
        } else {
            let tgcd = gcd(&i, &n);
            let s = (i / tgcd) as f64;
            let t = (n / tgcd) as f64;
            println!(
                "x = {}/{} PI, sin(x) = {:.6}, cos(x) = {:.6}",
                s,
                t,
                (s * PI / t).sin(),
                (s * PI / t).cos()
            );
        }
    }
}
