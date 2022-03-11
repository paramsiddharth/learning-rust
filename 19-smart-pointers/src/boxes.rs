use self::List::{Cons, Nil};

pub fn main() {
    let b = Box::new(1);
    println!("b = {}", b);

    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3, Box::new(Nil)))
        ))
    );
    println!("{:?}", list);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}