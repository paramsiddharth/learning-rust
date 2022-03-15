mod spawn;
#[path="./move.rs"]
mod move_;

fn main() {
    spawn::main();
    move_::main();
}