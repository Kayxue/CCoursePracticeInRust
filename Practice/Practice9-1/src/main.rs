use std::io::{self, Write};

fn a(m: i32, n: i32) -> i32 {
    if m == 0 {
        return n + 1;
    }
    if m > 0 && n == 0 {
        return a(m - 1, 1);
    }
    a(m - 1, a(m, n - 1))
}

fn main() {
    for i in 0..=3 {
        for k in 0..=9 {
            println!("A({}, {}) = {}", i, k, a(i, k));
        }
    }
}
