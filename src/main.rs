mod calc_method;
mod try_list;
mod try_hashmap;
mod try_if_else;
mod try_loop;
mod try_while;
mod try_for;

fn main( ) {
    println!("<< try calculations. >>");
    calc_method::test_type();
    println!("<< try Lists. >>");
    try_list::test_list();
    println!("<< try HashMap. >>");
    try_hashmap:: test_hashmap();

    println!("<< try add method.>>");
    let add_result1 = calc_method::add_method(30,102,8);
    println!("{:?}", add_result1);
    let (sum, mul) = calc_method::add_method(30,102,8);
    println!("{}, {}", sum, mul);

    println!("<< try swap method.>>");
    let(a, b) = calc_method::swap_method(3, 6);
    println!("a = {}, b = {}", a, b);

    println!("<< try if else>>");
    try_if_else::test_if_else();
    println!("<< try if else: calling and use return values of other methods.>>");
    try_if_else::test_math();

    println!("<< try loop>>");
    try_loop::test_loop();

    println!("<< try while>>");
    try_while::test_while();

    println!("<< try for >>");
    try_for::test_for();
}