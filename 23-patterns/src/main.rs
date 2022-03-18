#[allow(dead_code)]

fn main() {
    let x = 7;

    match x {
        1     => println!("One!"),
        2     => println!("Two!"),
        3     => println!("Three!"),
        5..=8 => println!("Five to eight."),
        _     => println!("Anything else.")
    }

    let x = Some(5);

    match x {
        Some(50) | Some(1) => println!("Fifty or one!"),
        Some(y)  => println!("This: {}", y),
        _        => println!("Default case. :(")
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("A to J."),
        'k'..='z' => println!("K to Z."),
        _         => println!("कहाँ-कहाँ से चले आते हैं यह लोग ?")
    }

    let p = Point {
        x: 0,
        y: 7
    };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x: 0, .. } => println!("'x' is 0."),
        Point { x, y: 0  } => println!("'y' is 0."),
        Point { x,    y  } => println!("x = {}, y = {}", x, y)
    }

    let msg = Message::ChangeColour(0, 160, 255);

    match msg {
        Message::Quit                  => println!("Exitting..."),
        Message::Move { x, y }         => println!("Moving to ({}, {}).", x, y),
        Message::Write(text)           => println!("Message: {}", text),
        Message::ChangeColour(r, g, b) => println!(
            "Red: {}, Green: {}, Blue: {}",
                   r,         g,        b
        )
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., second_last, last) => {
            println!("Some numbers: {}, .., {}, {}", first, second_last, last);
        }
    }

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello {
            id: id_var @ 3..=7,
        } => println!("Found an ID in range: {}", id_var),
        Message2::Hello { id: 10..=12 } => {
            println!("Found an ID in another range.")
        }
        Message2::Hello { id } => println!("Found some other ID: {}", id),
    }
}

struct Point {
    x: i32,
    y: i32
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move {
        x: i32,
        y: i32
    },
    Write(String),
    ChangeColour(i32, i32, i32)
}

enum Message2 {
    Hello { id: i32 },
}