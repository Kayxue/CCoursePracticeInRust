use std::io;
use std::io::Write;

fn main() {
    let mut dataCountStr = String::new();
    println!("Both A and B are matrices with m x n elements.");
    println!(" ");
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dataCountStr).unwrap();
    let dataCount: i32 = dataCountStr.trim().parse().unwrap();
    for z in 0..dataCount {
        let mut MNInput = String::new();
        print!("Input values of m and n: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut MNInput).unwrap();
        let splited = MNInput
            .trim()
            .split(" ")
            .map(|e| e.to_string())
            .collect::<Vec<_>>();
        let m: usize = splited[0].parse().unwrap();
        let n: usize = splited[1].parse().unwrap();
        let mut matrixA = vec![vec![0; n]; m];
        let mut matrixB = vec![vec![0; n]; m];
        let mut matrixC = vec![vec![0; n]; m];
        print!("Input numbers in matrix A: ");
        io::stdout().flush().unwrap();
        let mut inStrA = String::new();
        io::stdin().read_line(&mut inStrA).unwrap();
        let allNumA: Vec<i32> = inStrA
            .trim()
            .split(" ")
            .map(|e| e.parse().unwrap())
            .collect::<Vec<_>>();
        let mut now = 0;
        for i in &mut matrixA {
            for k in i {
                *k = allNumA[now];
                now += 1;
            }
        }
        print!("Input numbers in matrix B: ");
        io::stdout().flush().unwrap();
        let mut inStrB = String::new();
        io::stdin().read_line(&mut inStrB).unwrap();
        let allNumB: Vec<i32> = inStrB
            .trim()
            .split(" ")
            .map(|e| e.parse().unwrap())
            .collect::<Vec<_>>();
        let mut now = 0;
        for i in &mut matrixB {
            for k in i {
                *k = allNumB[now];
                now += 1;
            }
        }
        for (x, arr) in &mut matrixC.iter_mut().enumerate() {
            for (y, value) in arr.iter_mut().enumerate() {
                *value = matrixA[x][y] + matrixB[x][y];
            }
        }
        println!("Numbers in matrix C are:");
        print!("{{");
        for i in 0..matrixC.len() {
            print!("{{");
            for k in 0..matrixC[i].len() {
                print!("{}", matrixC[i][k]);
                if k != matrixC[i].len()-1 {
                    print!(", ");
                }
            }
            print!("}}");
            if i != matrixC.len()-1 {
                print!(",\n");
            }
        }
        print!("}}\n\n");
        io::stdout().flush().unwrap();
    }
}
