use std::io::{self, Write};

fn duplicateStr(dest: &mut String, src: String, repeatTimes: i32) {
    *dest = src.repeat(repeatTimes as usize);
}

fn main() {
    let mut repeatTimeStr = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut repeatTimeStr).unwrap();
    let repeatTime: i32 = repeatTimeStr.trim().parse().unwrap();
    for i in 0..repeatTime {
        let mut input = String::new();
        let mut newStr = String::new();
        let mut repString = String::new();
        print!("\nInput a string: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        print!("Duplicate how many times: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut repString).unwrap();
        let rep: i32 = repString.trim().parse().unwrap();
        duplicateStr(&mut newStr, input, rep);
        println!("The new string is [{}]", newStr);
    }
}
