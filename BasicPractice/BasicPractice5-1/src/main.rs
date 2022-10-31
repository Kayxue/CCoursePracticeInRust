use std::io::{self, Write};

const dayCount: [i8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn isLeap(year: &i32) -> bool {
    year % 400 == 0 || (year % 4 == 0 && !(year % 100 == 0))
}

fn isInvalidDate(year: &i32, month: &i32, day: &i32) -> bool {
    if *year <= 0 || *month <= 0 || *month > 12 || *day <= 0 {
        return true;
    }
    if *month == 2 {
        if isLeap(year) {
            if *day <= 29 {
                return false;
            } else {
                return true;
            }
        } else {
            if *day <= 28 {
                return false;
            } else {
                return true;
            }
        }
    } else {
        if *day <= dayCount[(*month - 1) as usize] as i32 {
            return false;
        } else {
            return true;
        }
    }
}

fn main() {
    let mut repeatTimesStr = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut repeatTimesStr).unwrap();
    let repeatTime: i32 = repeatTimesStr.trim().parse().unwrap();
    for i in 0..repeatTime {
        let mut dateStr = String::new();
        print!("\nInput a date (year/month/day): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut dateStr).unwrap();
        let dateArr: Vec<i32> = dateStr
            .trim()
            .split("/")
            .map(|e| e.parse().unwrap())
            .collect();
        let year = dateArr[0];
        let month = dateArr[1];
        let day = dateArr[2];
        let result = isInvalidDate(&year, &month, &day);
        if result {
            println!("{} is not a valid date.", dateStr.trim());
        } else {
            println!("{} is a valid date.", dateStr.trim());
        }
    }
}
