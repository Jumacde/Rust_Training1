pub fn test_method() {
    let s = String::from("hello world");
    let mut h = String::from("hello world!");
    println!("{}, {}" , s, s.len());
    println!("{}",h);
    h.push_str("A");
    println!("{}", h);
}