use std::io::{self, Write};

fn main() {
    let mut studentCountStr = String::new();
    let mut count: [i32; 12] = [0; 12];
    print!("請問有幾位同學？");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut studentCountStr).unwrap();
    let studentCount: i32 = studentCountStr.trim().parse().unwrap();
    println!("請輸入他們的生日 (格式為 年/月/日)。");
    for i in 0..studentCount {
        let mut dateString = String::new();
        io::stdin().read_line(&mut dateString).unwrap();
        let num: i32 = (dateString.split("/").collect::<Vec<&str>>())[1]
            .parse()
            .unwrap();
        count[(num - 1) as usize] += 1;
    }
    for (i, value) in count.into_iter().enumerate() {
        println!("{}月出生的有 {} 位", i + 1, value);
    }
}
