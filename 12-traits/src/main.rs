mod edible;
mod samosa;
mod chutney;
mod jalebi;
mod unknown;

use edible::eat;
use samosa::Samosa;
use chutney::Chutney;
use jalebi::Jalebi;
use unknown::Unknown;

fn main() {
    let food = Samosa::new(10);
    eat(&food);
    let food = Chutney::new(String::from("पुदीना"));
    eat(&food);
    let food = Jalebi::new(12.5);
    eat(&food);
    let food = Unknown::new();
    eat(&food);
}