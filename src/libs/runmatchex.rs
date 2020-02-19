#![allow(unused_variables)]
fn enum_checker_colors() {
    // `allow` required to silence warnings because only
    // one variant is used.
    #[allow(dead_code)]
    enum Color {
        // These 3 are specified solely by their name.
        Red,
        Blue,
        Green,
        // These likewise tie `u32` tuples to different names: color models.
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    // let color = Color::RGB(122, 17, 40);
    // let color = Color::HSV(102, 17, 40);
    // let color = Color::HSL(12, 117, 4);
    // let color = Color::CMY(211, 17, 5);
    let color = Color::CMYK(23, 1, 4, 88);
    // TODO ^ Try different variants for `color`

    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
        // Don't need another arm because all variants have been examined
    }


}

pub fn sample_coin_collection() {
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

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny");
                1
            },
            Coin::Nickel => {
                println!("Lucky Nickel");
                5
            },
            Coin::Dime => {
                println!("Lucky Dime ");
                10
            },
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    let penny = Coin::Penny;
    let quarter = Coin::Quarter;

    value_in_cents(penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(UsState::Alaska));
    value_in_cents(Coin::Quarter(UsState::Alabama));
    enum_checker_colors()
}