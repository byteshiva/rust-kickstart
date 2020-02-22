struct Point<T> {
    x: T,
    y: T,
}

pub fn generic_03() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("x in integer {:?} and float {:?}", integer.x, float.x);
    println!("y in integer {:?} and float {:?}", integer.y, float.y);
}
