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

    libs::runstructexample::structsample();

    libs::runenumexample::runenum();

    libs::runmatchex::sample_coin_collection();

    libs::flowcontrol::exmatch::runmatch_ex::runmatch_sample();

    libs::flowcontrol::exmatch::ex_06_match::run_plus_one();

    libs::flowcontrol::exmatch::ex_06_if_let::run_if_let_else();

    libs::generics::generics_ex_01::gen_ex_01();

    libs::generics::non_generic_02::run_generic_02();

    libs::generics::generic_03::generic_03();    
}
