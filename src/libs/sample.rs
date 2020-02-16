
pub fn hide() {
    println!("Hello, world!");
    let s = "abc012è€";
    for i in 0..s.len() {
        println!("{}: {}", i, s.as_bytes()[i]);
    }

    let mut r = "abc".chars();
    for i in r {
        r = "XY".chars();
        print!("{} {}; ", i, r.next().unwrap());
    }

    let slice1 = &[3, 4, 5];
    let slice2 = &[7, 8];
    println!("");
    println!("Slice 1 Begins");
    let mut iterator = slice1.iter();
    for item_ref in iterator {
        print!("[{}] ", *item_ref);
    }
    println!("");
    println!("Slice 2 Begins");

    iterator = slice2.iter();
    for item_ref in iterator {
        print!("({}) ", *item_ref);
    }
    println!("");

    let arr = [66, -8, 43, 19, 0, -31];
    for n in arr.iter() {
        if *n < 0 { print!("{} ", n); }
    }
    println!("");


    let arr = [66, -8, 43, 19, 0, -31];
    for n in arr.iter().filter(|x| **x < 0) {
        print!("{} ", n);
    }
    println!("");

    let arr = [66, -8, 43, 19, 0, -31];
    for n in arr.iter() {
        print!("{} ", n * 2);
    }
    println!("");

    let arr = [66, -8, 43, 19, 0, -31];
    for n in arr.iter().map(|x| *x * 2) {
        print!("{} ", n);
    }
    println!("");

    let arr = ['a', 'b', 'c'];
    for (i, ch) in arr.iter().enumerate() {
        print!("{} {}, ", i, *ch);
    }

    println!("");
    let arr = [45, 8, -2, 6];
    match arr.iter().min() {
        Some(n) => print!("{} ", n),
        _ => (),
    }

    println!("");

    match arr.iter().max() {
        Some(n) => print!("{} ", n),
        _ => (),
    }

    println!("");

    match [0; 0].iter().min() {
        Some(n) => print!("{} ", n),
        _ => print!("---"),
    }


    let arr = ["hello", "brave", "new", "world"];
    match arr.iter().min() {
        Some(n) => print!("{} ", n),
        _ => (),
    }

    match arr.iter().max() {
        Some(n) => print!("{} ", n),
        _ => (),
    }

    println!("");

    let arr = [36, 1, 15, 9, 4];
    let v = arr.iter().collect::<Vec<&i32>>();
    print!("{:?}", v);

    println!("");

    let s = "Hello";
    println!("{:?}",s.chars().collect::<String>());
    println!("{:?}",s.chars().collect::<Vec<char>>());
    println!("{:?}",s.bytes().collect::<Vec<u8>>());
    println!("{:?}",s.as_bytes().iter().collect::<Vec<&u8>>());
}