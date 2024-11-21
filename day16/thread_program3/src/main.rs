use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut data = data_clone.lock().unwrap();
            *data += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *data.lock().unwrap());
}
