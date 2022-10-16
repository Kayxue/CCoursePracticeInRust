use rand::{self, Rng};

fn main() {
    let mut count: [i32; 11] = [0; 11];
    for i in 0..6000 {
        let a = rand::thread_rng().gen_range(1..=6);
        let b = rand::thread_rng().gen_range(1..=6);
        count[(a + b) - 2] += 1;
    }
    for (index, value) in count.into_iter().enumerate() {
        println!("{} 出現過 {} 次", index + 2, value);
    }
}
