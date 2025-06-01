// pub fn bigger(a: i32, b: i32) -> i32 {
//     // Complete this function to return the bigger number!
//     // Do not use:
//     // - another function call
//     // - additional variables
//     return if a > b { a } else { b };
// }

// // Don't mind this for now :)
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn ten_is_bigger_than_eight() {
//         assert_eq!(10, bigger(10, 8));
//     }

//     #[test]
//     fn fortytwo_is_bigger_than_thirtytwo() {
//         assert_eq!(42, bigger(32, 42));
//     }
// }

// use num::integer::sqrt;

// // There's a prime number hiding in our array of integers!
// // The function below tries to find the prime number by checking each element,
// // and finding its divisor. If none is found, then it's a prime number and
// // its search ends!

// // But it seems that its search never does end, when there's clearly a
// // prime number there. Fix the function so that it returns the prime number.

// fn main() {
//     let numbers = [36, 25, 49, 3, 64, 16, 9];
//     let prime = get_prime(numbers);
// }

// fn get_prime(arr: [i32; 7]) -> i32 {
//     // Loop through every element in the array
//     let mut i = 0;
//     'outer: loop {
//         // Find a divisor.
//         let mut n = 2;
//         'inner: loop {
//             // If a number can be evenly divided by another number except 1 and itself,
//             // then it's not a prime.
//             // Break out here to move on to the next element.
//             if arr[i] % n == 0 {
//                 if arr[i] == 2 {
//                     break 'outer;
//                 }
//                 i += 1;
//                 break;
//             }

//             // If no divisors are found, then we've found a prime!
//             // Break out of the loop here.
//             if n >= sqrt(arr[i]) {
//                 break 'outer;
//             }

//             // Otherwise, move to the next element.
//             n += 1;
//         }
//     }
//     println!("The first prime number in the array is {}.", arr[i]);
//     arr[i]
// }

// // Below is the classic FizzBuzz program. It prints every number from 1 to 100,
// // except for multiples of 3 it prints "fizz" instead of the number, and for
// // multiples of 5 it prints "buzz" instead of the number. If the number is
// // both a multiple of 3 and 5, it prints "fizzbuzz".

// // Fix the compile time error so that the program runs successfully.

// fn main() {
//     let mut n = 1;
//     while n <= 100 {
//         if n % 15 == 0 {
//             println!("fizzbuzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else if n % 5 == 0 {
//             println!("buzz");
//         } else {
//             println!("{}", n);
//         }
//         n += 1;
//     }
// }

// Here's a bucket list of cities I'd like to visit one day, and I'd like to
// share it with the world. Fix the loop so it announces the cities I'd like to visit.

fn main() {
    let cities = ["Perth", "Qingdao", "Rome"];
    for city in cities {
        println!("I'd like to visit {} someday!", city);
    }
}
