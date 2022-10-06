use macros::sum;
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

/* impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
} */

fn main() {
	println!("{}", sum![1, 2, 3]);

	Pancakes::hello_macro();
}
