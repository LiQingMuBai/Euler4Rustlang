fn main() {
    println!("Hello, world!");
    let number:u128 = 13195;
    let mut n = 2;
    let mut sum = 0;
    loop {
    if number % n == 0 {
        println!("{}", n);
        sum = sum + n;
    }
    if n >100{
        println!("OK, that's enough");
        // 退出循环
        break;
    }
    n +=1;
    }

    println!("{}", sum);
}
