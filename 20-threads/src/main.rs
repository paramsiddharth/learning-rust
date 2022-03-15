mod spawn;
#[path="./move.rs"]
mod move_;
mod channels;

fn main() {
    spawn::main();
    move_::main();
    channels::main();
}