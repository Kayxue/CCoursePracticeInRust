use std::io::{self, Write};

fn c(n: i32, k: i32) -> i32 {
    if k == 0 {
        return 1;
    }
    if k == n {
        return 1;
    }
    c(n - 1, k - 1) + c(n - 1, k)
}

fn main() {
    for i in 1..=6 {
        for k in 0..=i {
            println!("C({}, {}) = {}", i, k, c(i, k));
        }
    }
}
