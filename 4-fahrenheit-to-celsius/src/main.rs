use std::io;
use std::io::Write;
use std::process;

fn main() {
    print!("Enter the temprature in degrees fahrenheit: ");

    io::stdout().flush().expect("Error displaying!");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Invalid temprature!");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    let temp = (9.0 / 5.0) * temp + 32.0;

    println!("Temperature in degrees celsius: {:.2}", temp);
}
