pub fn function() {
    println!("called `my::nested me::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `my::nested::private_function()`");
}

// // Modules can also be nested
// pub mod nested {
//     pub fn function() {
//         println!("called `libs::nested::function()`");
//     }

//     #[allow(dead_code)]
//     fn private_function() {
//         println!("called `libs::nested::private_function()`");
//     }

//     // Functions declared using `pub(in path)` syntax are only visible
//     // within the given path. `path` must be a parent or ancestor module
//     pub(in crate::libs) fn public_function_in_libs() {
//         print!("called `libs::nested::public_function_in_libs()`, that\n> ");
//         public_function_in_nested();
//     }

//     // Functions declared using `pub(self)` syntax are only visible within
//     // the current module, which is the same as leaving them private
//     pub(self) fn public_function_in_nested() {
//         println!("called `libs::nested::public_function_in_nested()`");
//     }

//     // Functions declared using `pub(super)` syntax are only visible within
//     // the parent module
//     pub(super) fn public_function_in_super_mod() {
//         println!("called `libs::nested::public_function_in_super_mod()`");
//     }
// }

// Nested modules follow the same rules for visibility
mod private_nested {
    #[allow(dead_code)]
    pub fn function() {
        println!("called `libs::private_nested::function()`");
    }

    // Private parent items will still restrict the visibility of a child item,
    // even if it is declared as visible within a bigger scope.
    #[allow(dead_code)]
    pub(crate) fn restricted_function() {
        println!("called `libs::private_nested::restricted_function()`");
    }
}