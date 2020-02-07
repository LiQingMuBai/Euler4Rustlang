fn main() {
    let mut count = 1;
    let mut sum = 0;
      loop {
        count += 1;
        if count % 3 == 0 || count % 5 == 0{
            sum = sum + count;
        }
        if count == 1000 {
            println!("OK, that's enough");
            break;
        }
    }

    println!("{}",sum);
}
