use std::io::{self, Write};

fn sort3(x: &mut i32, y: &mut i32, z: &mut i32) {
    if x > y {
        swap(x, y);
    }
    if x > z {
        swap(x, z);
    }
    if y > z {
        swap(y, z);
    }
}

fn swap(a: &mut i32, b: &mut i32) {
    (*a, *b) = (*b, *a);
}

fn main() {
    let mut repeatTimeStr = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut repeatTimeStr).unwrap();
    let repeatTime: i32 = repeatTimeStr.trim().parse().unwrap();
    for i in 0..repeatTime {
        let mut inputStr = String::new();
        print!("\nHow many sets of test data: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut inputStr).unwrap();
        let args: Vec<i32> = inputStr
            .trim()
            .split(" ")
            .map(|e| e.parse().unwrap())
            .collect();
        let (mut x, mut y, mut z) = (args[0], args[1], args[2]);
        sort3(&mut x, &mut y, &mut z);
        println!("Results after sorting: {}, {}, {}", x, y, z);
    }
}
