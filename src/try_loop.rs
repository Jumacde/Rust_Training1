pub fn test_loop() {
    let mut x: i32 = 8;
    loop {
        x -= 1;
        if x == 2 {
            break;
        }
    }
    println!{"{}", x}
}