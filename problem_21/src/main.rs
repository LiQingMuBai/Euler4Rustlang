fn main() {
    println!("Hello, world!");

    for n in 1..221 {
        println!("n is {} ", n);
        for i in 1..n / 2 {
            if n % i == 0 {
                println!("{}", i);
            }
        }
    }
}
