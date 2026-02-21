pub fn test_match() {
    let x: i32 = 201;
    match x {
        0 => {
            println!("x is {}", x );
        }

        1|3 => {
            println!("x is 1 or 3, so {}", x);
        }

        4..100 => {
            println!("x is between 4 and 100, so {}", x);
        }

        // x ia matched and converted to a new values y.
        y  @ 101..=200 => {

            println!("x is between 101 and 200, so its converted to y. {}", y)
        } 

        _ => {
            println!("x isnt number between 0 and 200. x = {}", x);
        }
    }

}