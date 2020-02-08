fn main() {
    println!("Hello, world!");
    let mut n:u32 = 100000;
    loop {
        n += 1;
        let array:Vec<u32> = n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
  
    if array[0]==array[5] && array[1]==array[4] && array[2]==array[3] {
        println!("{:?}",array);
    }
        // println!("{}", count);
        if n == 999999 {
            println!("OK, that's enough");

            // 退出循环
            break;
        }
    }
}
