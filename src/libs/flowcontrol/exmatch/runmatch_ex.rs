pub fn runmatch_sample() {
    
    fn find_num(val: u32){
        // TODO ^ Try different values for `number`
        let number = val;
        println!("------------  Start  ---------------");
        println!("Tell me about {}", number);
        match number {
            // Match a single value
            1 => println!("One!"),
            // Match several values
            2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
            // Match an inclusive range
            13..=19 => println!("A teen"),
            // Handle the rest of cases
            _ => println!("Ain't special"),
        }
        println!("***********   END     **************");
    }

    find_num(2);
    find_num(1);
    find_num(12);
    find_num(3);
    find_num(19);
    find_num(21);

    let boolean = false;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);
}