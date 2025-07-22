// TODO: Fix the compiler error by adding one or two characters.
#[rustfmt::skip]
macro_rules! my_macro {
    // This macro prints a message when called.
    // It can also take an expression and print it.
    // The first arm is for the empty case, and the second arm is for the case
    // where an expression is provided.


    () => {
        println!("Check out my macro!");
    }
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
