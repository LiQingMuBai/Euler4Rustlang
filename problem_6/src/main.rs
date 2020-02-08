fn main() {
    println!("Hello, world!");
}
fn main() {
    println!("Hello, world!");
    let mut count_1 = 1;
    let mut count_0 = 1;
    let mut sum_0 = 0;
    let mut sum_1 = 0;
    loop {
        sum_0 = sum_0 + count_0 * count_0;
        if count_0 == 100 {
            println!("OK, that's enough");
            break;
        }
        count_0 += 1;
    }
    loop {
        sum_1 = sum_1 + count_1;
        if count_1 == 100 {
            println!("OK, that's enough");
            break;
        }
        count_1 += 1;
    }
    sum_1 = sum_1 * sum_1;

    println!("{}",sum_0);
    println!("{}",sum_1);
}
