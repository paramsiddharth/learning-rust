fn main() {
    let param = Person {
        name: String::from("Param"),
        gender: Gender::Male
    };

    println!("{}", param);
}

#[allow(dead_code)]
enum Gender {
    Male,
    Female
}

struct Person {
    name: String,
    gender: Gender
}

impl Gender {
    fn to_string(&self) -> String {
        match &self {
            Gender::Male => String::from("Male"),
            Gender::Female => String::from("Female")
        }
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Person: {} ({})", self.name, self.gender.to_string())
    }
}