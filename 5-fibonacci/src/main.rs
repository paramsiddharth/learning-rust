use std::io;
use std::io::Write;
use std::process;

fn main() {
    print!("Enter the number: ");

    io::stdout().flush().expect("Failed to print!");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Invalid number!");

    let num: u32 = match num.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number!");
            process::exit(1);
        }
    };

    println!("Fibonacci ({}) = {}", num, fib(num));
}

fn fib(n: u32) -> u32 {
    if n < 2 { n } else { fib(n - 1) + fib(n - 2) }
}