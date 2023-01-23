fn main() {
    let letters = std::env::args().nth(1).expect("letters expected");
    println!("values sum: {}", lettersum(&letters));
}

fn lettersum(s: &str) -> i32 {
    let a = 'a';
    let z = 'z';

    s.chars()
        .filter(|c| c.ge(&a) && c.le(&z))
        .map(|c| c as i32 - a as i32 + 1)
        .sum()
}