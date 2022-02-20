pub fn main() {
    let a = "Hi!".to_string();
    let b = String::from("Bye!");
    println!("{} {}", a, b);

    let c = format!("{} {}", a, b);
    println!("{}", c);

    let d = "नमस्कार ।".to_string();
    println!("{}", d);
    for i in b.bytes() {
        print!("{} ", i);
    }
    println!();
}