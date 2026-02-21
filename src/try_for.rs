pub fn test_for() {
    let mut a: i32 = 0;
    let mut b: i32 = 5;
    println!("show values between {} and 4", a);
    for i in a..b {
        println!("{}", i);
    }
    println!("----------------");
    println!("show values between {} and {}", a, b);
    for i in a ..= b {
        println!("{}", i);
    }
}