fn main() {
    let param = build_person(String::from("Param"), 20);
    println!("{} {} {}", param.name, param.age, param.count);
    
    // Here, the ownership of 'param' is lost to the function
    let param_2 = extend_person(param, 99);
    println!("{} {} {}", param_2.name, param_2.age, param_2.count);
}

struct Person {
    name: String,
    age: u8,
    count: u32
}

fn build_person(name: String, age: u8) -> Person {
    Person {
        name,
        age,
        count: 1
    }
}

fn extend_person(person: Person, age: u8) -> Person {
    Person {
        age,
        count: 1,
        ..person
    }
}