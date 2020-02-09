fn main() {
    let mut inner = 1;
    let mut out = 1;
    for i in 1..11 {
        inner = 1;
        for j in 1..i {
            inner = inner * j;
        }
        println!("{}",inner);
    }
}
