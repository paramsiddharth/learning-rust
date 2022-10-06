use advanced_traits::{Point, Human, Pilot, Wizard, Wrapper};

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 }
        + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    println!("Success!");

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    let w = Wrapper(vec![
        String::from("Hello"),
        String::from("World!")
    ]);
    println!("w = {}", w);
}
