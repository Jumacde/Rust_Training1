pub fn test_if_else() {
    let x: i32 = 16;
    if x < 16 {
        println!("you can´t drink bier.");
    } else if x > 16 {
        println!("you can drink bier.");
    } else { 
        println!("you can drink bier but be careful.");
    }
}

// test calling and use return values of other methods.
pub fn add_meth(x: i32, y: i32)-> i32 {
    return x + y;
} 

pub fn sub_meth(x: i32, y:i32) -> i32 {
    return y - x;
}


pub fn test_math() {
    let x:i32 = 9;
    let y:i32 = 7;
    if x > y {
        println!("x, {}, > y, {}, also " , x, y);
        let sum = add_meth(x, y);
        println!("{} +  {} = {}", x, y, sum);
    } else if x < y {
        println!("x, {}, < y, {}, also " , x, y);
        let sub = sub_meth(x, y);
        println!("{} -  {} = {}", y, x, sub);
    } else if x == y {
        println!("x, {}, = y, {}, also " , x, y);
        let mul = crate::calc_method::mul_meth(x, y);
        println!("{} *  {} = {}", x, y, mul);
    } else {
        println!("finish.");
    }

}