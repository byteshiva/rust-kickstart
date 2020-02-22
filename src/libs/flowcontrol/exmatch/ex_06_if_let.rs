#![allow(unused_variables)]
#![allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn some_u8_value_ex() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
}

fn some_u8_value_if_let_ex() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn coin_check_match_ex() {
    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    println!("count match - count {:?}", count);
}

fn coin_check_if_let_else_ex() {
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
        println!("if let - count {:?}", count);
    }
    // count;
}

pub fn run_if_let_else() {
    some_u8_value_ex();
    some_u8_value_if_let_ex();
    coin_check_match_ex();
    coin_check_if_let_else_ex();
}