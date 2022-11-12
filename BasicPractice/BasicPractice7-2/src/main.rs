use std::io::{self, Write};

fn mya(k: i32, m: i32, n: i32, s: i32, t: i32, a: i32, finalIndex: i32) {
    if k > finalIndex {
        return;
    }
    println!("a_{} = {}", k, a);
    if k % 2 != 0 {
        mya(k + 1, m, n, s, t, s * a + t, finalIndex);
    } else {
        mya(k + 1, m, n, s, t, m * a + n, finalIndex);
    }
}

fn main() {
    let mut dataCountStr = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dataCountStr).unwrap();
    let dataCount: i32 = dataCountStr.trim().parse().unwrap();
    for i in 0..dataCount {
        let mut mStr = String::new();
        let mut nStr = String::new();
        let mut sStr = String::new();
        let mut tStr = String::new();
        let mut finalIndexStr = String::new();
        print!("\nm = ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut mStr).unwrap();
        print!("n = ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut nStr).unwrap();
        print!("s = ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut sStr).unwrap();
        print!("t = ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut tStr).unwrap();
        print!("Final index = ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut finalIndexStr).unwrap();
        let m: i32 = mStr.trim().parse().unwrap();
        let n: i32 = nStr.trim().parse().unwrap();
        let s: i32 = sStr.trim().parse().unwrap();
        let t: i32 = tStr.trim().parse().unwrap();
        let finalIndex: i32 = finalIndexStr.trim().parse().unwrap();
        mya(0, m, n, s, t, 0, finalIndex);
    }
}
