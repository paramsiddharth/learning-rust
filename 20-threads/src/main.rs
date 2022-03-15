mod spawn;
#[path="./move.rs"]
mod move_;
mod channels;
mod mutexes;

fn main() {
    spawn::main();
    move_::main();
    channels::main();
    mutexes::main();
}