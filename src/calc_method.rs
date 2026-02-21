pub fn test_type() {
    // learn basic function-structure and variables types.
    let a : u8 = 26;
    let b : u8 = 15;
    let sum1: u8 = a + b;

    let x : u8 = 20;
    let y : u32 = 19;
    let sub1: u8 = x - y as u8; 

    let i : i8 = 12 ;
    let j : i8 = -14;
    let sub2: i8 = i -j;
    let k : i32 = 124;
    let sub3: i8 = i - k as i8;

    let m : f32 = 1.19;
    let n : f64 = 3.333;
    let o : f64 = -8.45;
    let fsum1: f32 = m + n as f32; 
    let fsum2: f32 = m + a as f32;
    let fsub1 : f64 = o - m as f64; 
    let fsub2 : f64 = m as f64 - o; 

    // try add

    println!("{} + {} = {}", a, b, sum1);
    println!("{} - {} = {}", x, y, sub1);
    println!("{} - ({}) = {}", i, j, sub2);
    println!("{} - {} = {}", i, k, sub3);
    println!("{} + {} = {}", m, n, fsum1);
    println!("{} + {} = {}", m, a, fsum2);
    println!("{} - {} = {}", o, m, fsub1);
    println!("{} - ({}) = {}", m, o, fsub2);
}
