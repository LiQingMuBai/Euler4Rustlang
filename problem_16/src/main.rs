extern crate num_bigint;
use num_bigint::BigUint;
fn main() {
    println!("Hello, world!");
    let mut value = BigUint::from(1 as u64);
    for _ in 1..1000 {
        value *= BigUint::from(2 as u64);
    }
    let full_str = value.to_string();
    println!("{}", full_str);
    let s = full_str
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .sum::<u32>();
	println!("{}", s);
}

