// generic-struct.rs
#[derive(Debug)]
#[allow(dead_code)]
struct Money<T> {
    amount: T,
    currency: String,
}
pub fn generic_struct_01() {
    let whole_euros: Money<u8> = Money {
        amount: 42,
        currency: "EUR".to_string(),
    };
    let floating_euros: Money<f32> = Money {
        amount: 24.312,
        currency: "EUR".to_string(),
    };
    println!("Whole euros: {:?}", whole_euros);
    println!("Floating euros: {:?}", floating_euros);
}
