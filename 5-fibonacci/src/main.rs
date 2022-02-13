use std::io;
use std::io::Write;
use std::process;
use std::env;

fn main() {
    let mut num = String::new();
    
    if env::args().len() < 2 {
        print!("Enter the number: ");
    
        io::stdout().flush().expect("Failed to print!");


        io::stdin()
            .read_line(&mut num)
            .expect("Invalid number!");
    } else {
        let args: Vec<String> = env::args().collect();
        num = args[1].to_string();
    }

    let num: u32 = match num.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number '{}'!", num);
            process::exit(1);
        }
    };

    println!("Fibonacci ({}) = {}", num, fib(num));
}

fn fib(n: u32) -> u32 {
    if n < 2 { n } else { fib(n - 1) + fib(n - 2) }
}