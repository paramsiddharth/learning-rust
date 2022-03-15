use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hi!"),
            String::from("Hello!"),
            String::from("नमस्ते ।"),
            String::from("प्रणाम ।")
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("Bye!"),
            String::from("Sayonara!"),
            String::from("अलविदा ।"),
            String::from("फिर मिलेंगे ।")
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for rv in rx {
        println!("Message: {:?}", rv);
    }
}