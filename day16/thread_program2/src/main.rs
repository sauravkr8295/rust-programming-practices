use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0)); // Wrap Mutex in Arc for shared ownership
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // Clone Arc for each thread
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // Lock the mutex
            *num += 1; // Increment the counter
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // Wait for all threads to finish
    }

    println!("Result: {}", *counter.lock().unwrap()); // Access final value
}















// use std::rc::Rc;
// use std::sync::Mutex;
// use std::thread;

// fn main() {
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter = Rc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }