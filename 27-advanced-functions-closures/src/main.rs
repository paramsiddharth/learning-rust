fn square(x: i32) -> i32 {
    x.pow(2)
}

fn do_twice<T>(f: fn(T) -> T, x: T) -> T {
    f(f(x))
}

fn get_closure() -> Box<dyn Fn(&i32) -> String> {
    Box::new(|x| "न॰ ".to_owned() + &x.to_string())
}

fn main() {
    let num = 2;
    let answer = do_twice(square, num);

    println!("({} ^ 2) ^ 2 = {}", num, answer);

    let closure = get_closure();
    let numbers: Vec<i32> = (1..=100).collect();
    let string: Vec<String> = numbers.iter()
                        // .map(|x| x.to_string())
                        .map(closure)
                        .collect();
    let string = string.join(", ");
    println!("{}", string);
}
