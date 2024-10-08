use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}

// use hello_macro::HelloMacro;
// use hello_macro_derive::HelloMacro;

// #[derive(HelloMacro)]
// struct Pancakes;

// fn main() {
//     Pancakes::hello_macro();
// }
