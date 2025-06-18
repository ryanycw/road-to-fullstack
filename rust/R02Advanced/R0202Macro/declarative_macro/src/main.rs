// // Fix the code to make it compile.

// macro_rules! my_macro {
//     () => {
//         println!("Check out my macro!");
//     };
// }

// fn main() {
//     my_macro!();
// }

// // Everything seems correct, but the code does not compile. Maybe it has to do with the position of defining a macro.

// macro_rules! my_macro {
//     () => {
//         println!("Check out my macro!");
//     };
// }

// fn main() {
//     my_macro!();
// }

// // Fix the code by bringing `my_macros` in scope (You have to mark `macros` module with something).

// mod macros {
//     #[macro_export]
//     macro_rules! my_macro {
//         () => {
//             println!("Check out my macro!");
//         };
//     }
// }

// fn main() {
//     my_macro!();
// }

// // Fix the code to make it compile. You can not remove anything.

// #[rustfmt::skip]
// macro_rules! my_macro {
//     () => {
//         println!("Check out my macro!");
//     };
//     ($val:expr) => {
//         println!("Look at this other macro: {}", $val);
//     };
// }

// fn main() {
//     my_macro!();
//     my_macro!(7777);
// }

// // Complete the definition of `sum`.

// macro_rules! sum {
//     ($($val:expr),*) => {
//         {
//             let mut sum = 0;
//             $( sum += $val; )+
//             sum
//         }
//     };
// }

// fn main() {
//     assert_eq!(sum!(1, 2, 3), 6);
//     assert_eq!(sum!(10u8, 20u8), 30);
// }

// We are trying to create a macro called `vec2`, which has the same functionality as the `vec` macro.
// Complete the transcriber for each matcher.

macro_rules! vec2 {
    () => {
        vec![]
    };
    ($($a:expr),+ $(,)?) => {{
        vec![$($a),+]
    }};
    ($m:expr; $n:expr) => {{ vec![$m; $n] }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn creates_empty_vector() {
        let first: Vec<i32> = vec![];
        let second: Vec<i32> = vec2![];
        assert_eq!(first, second);
    }

    #[test]
    fn creates_vector_from_list() {
        assert_eq!(vec![1, 2, 3,], vec2![1, 2, 3,]);
        assert_eq!(vec!['a', 'b', 'b', 'a'], vec2!['a', 'b', 'b', 'a']);
    }

    #[test]
    fn creates_vector_with_repeating_element() {
        assert_eq!(vec![5; 10], vec2![5;10]);
    }
}

fn main() {}
