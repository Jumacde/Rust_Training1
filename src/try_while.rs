pub fn test_while() {
    let mut x: i32 = 7;
    let init_x: i32 = x;
    let y: i32 = 4;
    let mut count:i32 = 0; 
    println!("x = {}", x);
    while x != y {
        x -= 1;
        count += 1;
    }
    println!("then {} - 1 until {}. {} times minus 1. x = {}",init_x, y, count, x);
}