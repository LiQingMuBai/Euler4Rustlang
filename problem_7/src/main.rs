fn main() {
   
    let mut index = 0;
    let mut number = 3;
    loop {
       
        if number%2 != 0 {
            println!("{}",number);
            index = index + 1;
        }
        number = number + 1;
        if index == 10_001 {
            println!("OK, that's enough");
            break;
        }

    }
}
