// This declaration will look for a file named `my.rs` or `my/mod.rs` and will
// insert its contents inside a module named `my` under this scope
mod libs;

fn function() {
    println!("called `function()`");
}

fn main() {


    libs::function();

    function();

    libs::indirect_access();

    libs::nested::function();

    libs::sample::hide();
}
