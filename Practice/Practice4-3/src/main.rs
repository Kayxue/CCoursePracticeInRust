use std::io::{self, Write};

fn main() {
    let beginningDay = [20, 20, 21, 21, 21, 22, 23, 23, 23, 23, 22, 22];
    let zodiacName = [
        "牡羊", "金牛", "雙子", "巨蟹", "獅子", "處女", "天秤", "天蠍", "射手", "摩羯", "水瓶",
        "雙魚",
    ];
    let mut dateStr = String::new();
    print!("請輸入日期 (以 月/日 的格式)：");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dateStr).unwrap();
    let splited: Vec<i8> = dateStr
        .trim()
        .split("/")
        .map(|e| e.parse().unwrap())
        .collect::<Vec<_>>();
    let month = splited[0];
    let day = splited[1];
    let mut zodiacCode = (month + 9) % 12;
    if day < beginningDay[(month - 1) as usize] {
        zodiacCode = (zodiacCode + 11) % 12;
    }
    println!(
        "你是{}座的，星座代碼是 {}。",
        zodiacName[zodiacCode as usize], zodiacCode
    );
}
