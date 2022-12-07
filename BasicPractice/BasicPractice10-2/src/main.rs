use regex::Regex;
use std::io::{self, Write};

fn keywordMasking(sentence: &mut String, keyword: String) {
    let regex = Regex::new(&keyword).unwrap();
    let mask = "#".repeat(keyword.len());
    *sentence = regex.replace_all(&sentence, mask).to_string();
}

fn main() {
    let mut repeatTimeStr = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut repeatTimeStr).unwrap();
    let repeatTime: i32 = repeatTimeStr.trim().parse().unwrap();
    for i in 0..repeatTime {
        let mut sentence = String::new();
        let mut keyword = String::new();
        print!("\nInput a sentence: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut sentence).unwrap();
        sentence = sentence.trim().to_string();
        print!("Input a keyword: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut keyword).unwrap();
        keyword = keyword.trim().to_string();
        keywordMasking(&mut sentence, keyword);
        println!("The masked sentence is [{}]", sentence);
    }
}
