// // Inorder to access a value wrapped in a mutex, it's lock has to be acquired.
// // Fix the code by acquiring the lock at appropriate places.

// use std::sync::{Arc, Mutex};
// use std::thread;

// struct Wrapper {
//     value: i32,
// }

// impl Wrapper {
//     fn new() -> Self {
//         Wrapper { value: 0 }
//     }
//     fn add(&mut self, to_add: i32) {
//         self.value += to_add;
//     }
// }

// // calculate sum of range 1..=40000 using four threads
// fn main() {
//     let sum = Arc::new(Mutex::new(Wrapper::new()));
//     let mut handles = Vec::new();
//     for i in 0..4 {
//         let sum_clone = Arc::clone(&sum);
//         let handle = thread::spawn(move || {
//             let mut sum = 0;
//             let start = i * 10000 + 1;
//             for num in start..start + 10000 {
//                 sum += num;
//             }
//             // TODO: acquire lock and add sum to sum_clone
//             let mut sum_lock = sum_clone.lock().unwrap();
//             sum_lock.add(sum);
//         });
//         handles.push(handle);
//     }
//     for handle in handles {
//         handle.join().unwrap();
//     }
//     // TODO: acquire lock and print the sum value
//     println!("Sum of range 1..=40000 : {}", sum.lock().unwrap().value);
// }

// Fix the code to make it compile.

use std::sync::{Arc, Mutex};
use std::thread;

fn is_prime(num: u32) -> bool {
    for i in 2..=num / 2 {
        if num % i == 0 {
            return false;
        }
    }
    if num <= 1 { false } else { true }
}

// list of all prime numbers less than 10000 using four threads
fn main() {
    let primes = Arc::new(Mutex::new(Vec::new()));
    let thread_count = 4;
    let elemets_per_thread = 10000 / thread_count;
    let mut handles = Vec::new();
    for i in 0..thread_count {
        let start = i * elemets_per_thread;
        let list_clone = Arc::clone(&primes);
        let handle = thread::spawn(move || {
            for num in start..start + elemets_per_thread {
                if is_prime(num) {
                    let mut lock = list_clone.lock().unwrap();
                    lock.push(num);
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let lock = primes.lock().unwrap();
    println!("Prime numbers:");
    println!("{:?}", lock);
    assert_eq!(lock.len(), 1229);
}
