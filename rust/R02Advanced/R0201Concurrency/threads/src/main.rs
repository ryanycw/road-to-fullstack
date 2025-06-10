// // Modify the code to ensure that main thread waits for other threads to finish.

// use std::thread;

// fn main() {
//     // print hello 10 times from spawned thread
//     let handle = thread::spawn(|| {
//         for i in 0..10 {
//             println!("{i} Hello from spawned thread!")
//         }
//     });

//     // print hello 10 times from main thread
//     for i in 0..10 {
//         println!("{i} Hello from main thread!");
//     }

//     handle.join().unwrap();
// }

// // Fix the code to make sure that sum is calculated correctly.

// use std::{
//     ops::Range,
//     thread::{self, JoinHandle},
// };

// fn summation_thread(range: Range<u64>) -> JoinHandle<u64> {
//     thread::spawn(|| {
//         let mut sum = 0;
//         for num in range {
//             sum += num;
//         }
//         sum
//     })
// }

// // calculate the sum of numbers 1..=40_00_000 using four threads
// fn main() {
//     let handle1 = summation_thread(1..10_00_000);
//     let handle2 = summation_thread(10_00_000..20_00_000);
//     let handle3 = summation_thread(20_00_000..30_00_000);

//     let mut sum = 0u64;
//     for num in 30_00_000..=40_00_000 {
//         sum += num;
//     }

//     sum += handle1.join().unwrap();
//     sum += handle2.join().unwrap();
//     sum += handle3.join().unwrap();

//     println!("Sum of numbers 1..=40_00_000: {sum}");
//     assert_eq!(sum, 8000002000000);
// }

// // When main thread terminates, all the spawned threads also get terminated.
// // Make the main thread sleep for half a second before terminating, to ensure that spawned thread gets enough time to complete.
// // Do not join the spawned thread.

// use std::{thread, time::Duration};

// fn main() {
//     thread::spawn(|| {
//         println!("Count down from 10 to 1 🚀");
//         for num in (1..=10).rev() {
//             println!("Count down: {num}!");
//         }
//     });
//     println!("Count up from 1 to 10...");
//     for num in 1..=10 {
//         println!("Count up: {num}");
//     }
//     thread::sleep(Duration::from_millis(500));
// }

// // Fix the code to make it compile.

// use std::thread;

// fn main() {
//     let msg = "Hello from spawned thread".to_owned();
//     let handle = thread::spawn(move || {
//         println!("{msg}");
//     });
//     handle.join().unwrap();
// }

// // Fix the code to make it compile and produce the correct output.

// use std::sync::mpsc::{self, Sender};
// use std::thread;

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     let nums = [12, 43, 54, 43, 53, 52, 98, 89];
//     sum(&nums, 3, tx);
//     let mut res = 0;
//     for sum in rx {
//         res += sum;
//     }
//     println!("Sum of numbers: {res}");
// }

// // calculate sum of numbers using specified number of threads
// fn sum(nums: &[i32], thread_count: usize, tx: Sender<i32>) {
//     let elements_per_thread = nums.len() / thread_count;
//     let mut start_pos;
//     let mut partition;
//     for i in 0..thread_count - 1 {
//         start_pos = i * elements_per_thread;
//         partition = Vec::from(&nums[start_pos..start_pos + elements_per_thread]);
//         let tx_clone = tx.clone();
//         thread::spawn(move || {
//             let mut sum = 0;
//             for num in partition {
//                 sum += num;
//             }
//             tx_clone.send(sum).unwrap();
//         });
//     }
//     // sum the remaining elements using last thread
//     partition = Vec::from(&nums[(thread_count - 1) * elements_per_thread..]);
//     thread::spawn(move || {
//         let mut sum = 0;
//         for num in partition {
//             sum += num;
//         }
//         tx.send(sum).unwrap()
//     });
// }

// Fix the code to make it compile.

use std::sync::mpsc;
use std::thread;

fn main() {
    let sentences = [
        "!tpircs llehs a eb ot detnaw eh esuaceB ?tsuR nrael barC eht sirreF did yhW".to_owned(),
        "!sgel sih fo thgie lla htiw tsuR ni edoc nac eh - reksat-itlum etamitlu eht si barC eht sirreF".to_owned()
    ];
    let (tx, rx) = mpsc::channel();
    for sentence in sentences {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let reversed = sentence.chars().rev().collect::<String>();
            tx_clone.send(reversed).unwrap();
        });
    }
    drop(tx);
    let printer = thread::spawn(|| {
        println!("Reversed sentences:");
        for sentence in rx {
            println!("{sentence}");
        }
    });
    printer.join().unwrap();
}
