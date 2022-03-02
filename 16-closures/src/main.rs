use std::thread;
use std::time::Duration;

fn main() {
    let user_specified_value = 10;
    let random_number = 7;

    generate_workout(user_specified_value, random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} push-ups!",
            expensive_calculation(intensity)
        );
        println!(
            "Today, do {} sit-ups!",
            expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_calculation(intensity)
            );
        }
    }
}

fn expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}