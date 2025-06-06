// //  Make the code compile by filling in the `???`s

// fn main() {
//     let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

//     let mut my_iterable_fav_fruits = my_fav_fruits.iter(); // TODO: Step 1

//     assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
//     assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple")); // TODO: Step 2
//     assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
//     assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach")); // TODO: Step 3
//     assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
//     assert_eq!(my_iterable_fav_fruits.next(), None); // TODO: Step 4
// }

// // Make the code compile by only modifying the loop.

// fn main() {
//     let mut nums = [0, 1, 2, 3, 4];
//     let odd_nums = [1, 3, 5, 7, 9];
//     for num in nums.iter_mut() {
//         *num = 2 * *num + 1;
//     }
//     assert_eq!(nums, odd_nums)
// }

// Fix the code to make it compile.

use std::collections::HashMap;

fn main() {
    // marks scored out of 50
    let mut marks = HashMap::from([("Harry", 40.0), ("Hermoine", 50.0), ("Ron", 35.5)]);
    // convert marks into percentage
    for (_, marks) in &mut marks {
        *marks = (*marks * 100.0) / 50.0;
    }
    marks
        .iter()
        .for_each(|(student, marks)| println!("{student} scored {marks}%"));
}
