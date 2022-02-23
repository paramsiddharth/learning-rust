pub trait Edible {
    fn eat(&self) {
        println!("Hmmm...");
    }
}

pub fn eat(item: &impl Edible) {
    item.eat()
}