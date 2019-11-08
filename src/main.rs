// $Id$

extern crate rand;
use rand::Rng;

fn r(x: i64) -> i64 {
    x & (std::i64::MAX - 1)
}
fn main() {
    let b62digits = String::from("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let mut rng = rand::thread_rng();
    let mut rng63 = r(rng.gen::<i64>());
    let mut str = String::from("_");
    while rng63 != 0 {
        let d: i64 = rng63 % 62;
        rng63 = rng63 / 62;
        str.push(b62digits.chars().nth(d as usize).unwrap());
    }

    println!("Random object id: {}", str);
}
