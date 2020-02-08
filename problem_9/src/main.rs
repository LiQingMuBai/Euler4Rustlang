fn main() {
    for a in 1..1000 {
        for b in 1..1000 {
            for c in 1..1000 {
                if (a + b + c == 1000) && (a * a + b * b == c * c) && (a < b) {
                    println!("a is {}", a);
                    println!("b is {}", b);
                    println!("c is {}", c);
                }
            }
        }
    }
}
