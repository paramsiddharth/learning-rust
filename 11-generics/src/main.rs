use std::ops;

fn main() {
    let _a = sum(1, 2);
    let _a = sum(1 as f32, 2.0);
}

fn sum<T: ops::Add + ops::Add<Output = T>>(a: T, b: T) -> T {
    return a + b;
}