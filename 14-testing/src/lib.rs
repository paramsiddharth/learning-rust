#[macro_use]
extern crate derive_more;

#[cfg(test)]
mod tests;

#[derive(Debug)]
#[derive(Display)]
#[display(fmt = "{{height: {}, width: {}}}", height, width)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    #[allow(dead_code)]
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height >= other.height
                // ^ Here, it should be >=, not >.
    }
}

#[allow(dead_code)]
fn do_something(todo: String) {
    if todo.to_lowercase() == String::from("nothing") {
        panic!("Can't do nothing, mate!");
    }
    
    println!("{}", todo);
}