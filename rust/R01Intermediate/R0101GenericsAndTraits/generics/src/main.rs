// // Fix the code by annotating variable with the correct type.

// fn main() {
//     let mut shopping_list: Vec<&str> = Vec::new();
//     shopping_list.push("milk");
// }

// // Define the generic struct Point.
// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let p1 = Point { x: 20, y: 10 };
//     let p2 = Point { x: 22.3, y: 3.14 };
//     println!("Point1: ({}, {})", p1.x, p1.y);
//     println!("Point2: ({}, {})", p2.x, p2.y);
// }

// // Make the code compile by defining Operation enum with a single generic type.
// enum Operation<T> {
//     Add(T, T),
//     Mul(T, T),
//     Sub { left: T, right: T },
//     Div { dividend: T, divisor: T },
// }

// fn main() {
//     let _op1 = Operation::Add(15u8, 10u8);
//     let _op2 = Operation::Mul(150, 23);
//     let _op3 = Operation::Sub {
//         left: 120,
//         right: 50,
//     };
//     let _op4 = Operation::Div {
//         dividend: 10.23,
//         divisor: 2.43,
//     };
// }

// // Implement the add method for Pair<i32> type.

// struct Pair<T>(T, T);

// impl Pair<i32> {
//     fn add(&self) -> i32 {
//         self.0 + self.1
//     }
// }

// fn main() {
//     let p1 = Pair(10, 23);
//     let addition = p1.add();
//     assert_eq!(addition, 33);
// }

// // Rewrite Wrapper struct so that it supports wrapping ANY type.

// struct Wrapper<T> {
//     value: T,
// }

// impl<T> Wrapper<T> {
//     pub fn new(value: T) -> Self {
//         Wrapper { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn store_u32_in_wrapper() {
//         assert_eq!(Wrapper::new(42).value, 42);
//     }

//     #[test]
//     fn store_str_in_wrapper() {
//         assert_eq!(Wrapper::new("Foo").value, "Foo");
//     }
// }

// fn main() {}

// Fix the code so that it compiles.

fn take_and_give_ownership<T>(input: T) -> T {
    input
}

struct User {
    name: String,
    id: u32,
}

fn main() {
    let str1 = String::from("Ferris the 🦀!");
    let user1 = User {
        name: "Alice".to_string(),
        id: 199,
    };
    let _str2 = take_and_give_ownership(str1);
    let _user2 = take_and_give_ownership(user1);
}
