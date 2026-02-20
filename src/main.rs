fn main() {
    // 1. learn basic function-structure and variables types.
    let a : u8 = 26;
    let b : u8 = 15;
    let sum: u8 = a + b;

    let x : u8 = 20;
    let y : u32 = 19;
    let sub: u8 = x - y as u8; 

    let i : i8 = 12 ;
    let j : i8 = -14;
    let sub2: i8 = i -j;

    println!("{} + {} = {}", a, b, sum);
    println!("{} - {} = {}", x, y, sub);
    println!("{} - {} = {}", i, j, sub2);
}
