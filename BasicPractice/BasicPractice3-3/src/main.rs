use std::io;
use std::io::Write;

struct Parm {
    a: i32,
    b: i32,
    c: i32,
    r: i32,
    s: i32,
    t: i32,
}

struct ParmStr {
    a: String,
    b: String,
    c: String,
    r: String,
    s: String,
    t: String,
}

impl Parm {
    fn from(parmStr: ParmStr) -> Parm {
        Parm {
            a: parmStr.a.trim().parse().unwrap(),
            b: parmStr.b.trim().parse().unwrap(),
            c: parmStr.c.trim().parse().unwrap(),
            r: parmStr.r.trim().parse().unwrap(),
            s: parmStr.s.trim().parse().unwrap(),
            t: parmStr.t.trim().parse().unwrap(),
        }
    }
}

fn main() {
    let mut dataCountStr = String::new();
    print!("How many sets of test data: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dataCountStr).unwrap();
    let dataCount: i32 = dataCountStr.trim().parse().unwrap();
    for i in 0..dataCount {
        let mut parmStr: ParmStr = ParmStr {
            a: String::new(),
            b: String::new(),
            c: String::new(),
            r: String::new(),
            s: String::new(),
            t: String::new(),
        };
        let (mut xStr, mut yStr, mut nowxStr, mut nowyStr, mut nStr) = (
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        );
        print!("x1 = ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut xStr).unwrap();
        print!("y1 = ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut yStr).unwrap();
        print!("a = ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut parmStr.a).unwrap();
        print!("b = ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut parmStr.b).unwrap();
        print!("c = ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut parmStr.c).unwrap();
        print!("r = ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut parmStr.r).unwrap();
        print!("s = ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut parmStr.s).unwrap();
        print!("t = ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut parmStr.t).unwrap();
        print!("n = ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut nStr).unwrap();
        let (mut x, mut y, n): (i32, i32, i32) = (
            xStr.trim().parse().unwrap(),
            yStr.trim().parse().unwrap(),
            nStr.trim().parse().unwrap(),
        );
        let (mut nowx, mut nowy): (i32, i32);
        let parm = Parm::from(parmStr);
        for i in 0..n {
            println!("n = {}, (x, y) = ({}, {})", i + 1, x, y);
            nowx = parm.a * x + parm.b * y + parm.c;
            nowy = parm.r * x + parm.s * y + parm.t;
            x = nowx;
            y = nowy;
        }
        println!("");
    }
}
