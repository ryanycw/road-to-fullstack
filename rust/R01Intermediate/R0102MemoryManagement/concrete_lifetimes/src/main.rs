// // Fix the code by shifting only one statement.

// fn main() {
//     let str1 = "🦀".to_string();
//     println!("A crab: {str1}");
//     let bytes = str1.into_bytes();
//     println!("A crab represented in unicode: {bytes:?}");
// }

// // Fix the code by addressing the TODO.

// fn main() {
//     let num_ref;
//     let num = 23;
//     {
//         // TODO: shift below statement to appropriate location

//         num_ref = &num;
//     }
//     println!("Reference points to {}", num_ref);
// }

// Fix the code by shifting only one statement.

fn main() {
    let mut my_str = "Old String".to_owned();
    let ref1 = &my_str;
    println!("{ref1}");
    let ref2 = &mut my_str;
    ref2.replace_range(0..3, "New");
    println!("{ref2}");
}
