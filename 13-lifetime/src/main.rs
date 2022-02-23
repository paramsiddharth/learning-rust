fn main() {
    let a = "Param is a nice boy.";
    let b = "Henry is a gentleperson.";
    println!("{}", &greater(a, b));
}

fn greater<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}