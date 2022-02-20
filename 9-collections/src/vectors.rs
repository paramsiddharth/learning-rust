pub fn main() {
    let mut a: Vec<f32> = Vec::new();
    let mut b           = vec!['a', 'b', 'c'];

    a.push(1.0);
    b.push('d');

    println!("{:?}", &a);

    for i in &mut b {
        println!("{}", i);
        *i = (*i as u8 + 1) as char;
    }
    
    println!("{:?}", &b);

    let things = vec![
        Thing::String(String::from("Param")),
        Thing::Float(1.23),
        Thing::Int(4)
    ];

    for i in &things {
        println!("{:?}", i);
    }
}

#[derive(Debug)]
enum Thing {
    String(String),
    Float(f32),
    Int(i32)
}