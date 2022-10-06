static mut COUNTER: u32 = 0;

#[allow(unused_unsafe)]
fn main() {
    unsafe {
        println!("Hello, world!");
    }

    unsafe {
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    }

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("Value at r1: {}", *r1);
        println!("Value at r2: {}", *r2);
    }

    let address = 0x12345usize;
    let r = address as *const i32;

    unsafe {
        println!("Value at {}: {}", address, *r);
    }
}
