fn main() {
    let mut fib = vec![1, 2];
    let mut i = 2; 
    let mut sum = 2;
    loop {
        let c = fib[i - 1] + fib[i - 2];
        if c >= 4_000_000 {
            break;
        }
        fib.push(c);
        if c % 2 == 0 {
            sum += c;
        }
        i += 1;
    }
    println!("{}", sum);
}
