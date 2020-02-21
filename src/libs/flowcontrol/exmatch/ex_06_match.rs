fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn some_rand_match() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}


pub fn run_plus_one() {
    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(None);
    
    some_rand_match();

    println!("{:?}", six);
    println!("{:?}", _none);
}