// A module named `libs`
mod libs {
    // Items in modules default to private visibility.
    fn private_function() {
        println!("called `libs::private_function()`");
    }

    // Use the `pub` modifier to override default visibility.
    pub fn function() {
        println!("called `libs::function()`");
    }

    // Items can access other items in the same module,
    // even when private.
    pub fn indirect_access() {
        print!("called `libs::indirect_access()`, that\n> ");
        private_function();
    }

    pub fn call_public_function_in_libs() {
        print!("called `libs::call_public_function_in_libs()`, that\n> ");
        nested::public_function_in_libs();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `libs::public_function_in_crate()`");
    }


}