use std::io::{self, Write};

fn main() {
    let mut dayCountStr = String::new();
    let mut firstDayStr = String::new();
    print!("請問這個月有多少天？");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dayCountStr).unwrap();
    print!("請問這個月第一天是星期幾？(0=星期天, 1=星期一, ...) ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut firstDayStr).unwrap();
    let dayCount: i8 = dayCountStr.trim().parse().unwrap();
    let firstDay: i8 = firstDayStr.trim().parse().unwrap();
    let mut nowPrint: i8 = 1;
    let mut startPrint = false;
    println!(" S  M  T  W  T  F  S");
    while nowPrint <= dayCount {
        for i in 0..=6 {
            if nowPrint > dayCount {
                break;
            }
            if !startPrint {
                if i == firstDay {
                    print!("{:>2} ", nowPrint);
                    nowPrint += 1;
                    startPrint = true;
                } else {
                    print!("   ");
                }
            } else {
                print!("{:>2} ", nowPrint);
                nowPrint += 1;
            }
        }
        print!("\n");
    }
}
