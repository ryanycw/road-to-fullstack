// // Complete the code by deriving the required traits.

// #[derive(Default, Debug, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let my_point = Point { x: 20, y: 10 };
//     let origin = Point::default();
//     println!("Origin: {origin:?}");
//     if my_point == origin {
//         println!("Selected point is origin!");
//     } else {
//         println!("Selected point: {my_point:?}");
//     }
// }

// Only one trait needs to be derived. Can you figure out which?

#[derive(PartialEq)]
enum Size {
    Small,
    Medium,
    Large,
}

fn main() {
    let my_size = Size::Small;
    if my_size == Size::Small {
        println!("I can fit in any size!");
    }
}
