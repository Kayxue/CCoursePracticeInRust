use std::io::{self, Write};

fn main() {
    let mut studentCountStr = String::new();
    let mut scoreIntervalStr = String::new();
    print!("Number of students: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut studentCountStr).unwrap();
    print!("Score interval: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut scoreIntervalStr).unwrap();
    let (studentCount, scoreInterval): (i32, i32) = (
        studentCountStr.trim().parse().unwrap(),
        scoreIntervalStr.trim().parse().unwrap(),
    );
    let mut fieldCount = 0;
    for i in (0..=100).step_by(scoreInterval as usize) {
        fieldCount += 1;
    }
    let mut scores = vec![0; fieldCount];
    for i in 0..studentCount {
        let mut scoreStr = String::new();
        print!("Student#{}'s score: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut scoreStr).unwrap();
        let score: i32 = scoreStr.trim().parse().unwrap();
        scores[((score / scoreInterval) as usize)] += 1;
    }
    println!("\nScoreRange People BarChart");
    for i in 0..fieldCount {
        print!(
            "{:>3} ~ {:>3} {:>5}   ",
            i as i32 * scoreInterval,
            ((i + 1) as i32 * scoreInterval) - 1,
            scores[i]
        );

        for k in 0..scores[i] {
            print!("*");
        }
        print!("\n");
    }
    io::stdout().flush().unwrap();
}
