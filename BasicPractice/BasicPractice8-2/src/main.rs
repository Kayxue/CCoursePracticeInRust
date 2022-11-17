use std::io::{self, Write};

fn meanValue(a: &f64, b: &f64, aMean: &mut f64, gMean: &mut f64, hMean: &mut f64) {
    *aMean = (*a + *b) / 2.0;
    *gMean = (*a * *b).sqrt();
    if *a + *b == 0.0 {
        *hMean = 0.0
    } else {
        *hMean = (2.0 * *a * *b) / (*a + *b);
    }
}

fn main() {
    let mut dataCountStr = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dataCountStr).unwrap();
    let dataCount: i32 = dataCountStr.trim().parse().unwrap();
    for i in 0..dataCount {
        let mut inputStr = String::new();
        print!("\nPlease input two numbers: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut inputStr).unwrap();
        let args: Vec<f64> = inputStr
            .trim()
            .split(" ")
            .map(|e| e.parse().unwrap())
            .collect();
        let (a, b) = (args[0], args[1]);
        let (mut aMean, mut gMean, mut hMean) = (0f64, 0f64, 0f64);
        meanValue(&a, &b, &mut aMean, &mut gMean, &mut hMean);
        println!(
            "The arithmetic mean of {:.6} and {:.6} is {:.6}",
            a, b, aMean
        );
        println!(
            "The geometric mean of {:.6} and {:.6} is {:.6}",
            a, b, gMean
        );
        println!("The harmonic mean of {:.6} and {:.6} is {:.6}", a, b, hMean);
    }
}
