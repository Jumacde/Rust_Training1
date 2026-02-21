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

pub fn braak_loop() {
    let mut x :i32 = 0;
    let w = loop {
        x += 1;
        if x == 16 {
            println!("you can finally drink bier. {}", x,);
            break x;
         }
            
        };
        println!("{}", w);
    }
