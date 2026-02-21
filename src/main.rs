mod calc_method;
mod try_list;
mod try_hashMap;

fn main( ) {
    println!("<< try calculations. >>");
    calc_method::test_type();
    println!("<< try Lists. >>");
    try_list::test_list();
    println!("<< try HashMap. >>");
    try_hashMap:: test_hashmap();
}