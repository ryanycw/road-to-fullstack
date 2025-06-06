// // Fix the `fetch_last` function. Do not add any other statement.

// fn fetch_last<T>(list: &mut Vec<T>) -> Result<T, String> {
//     list.pop().ok_or("Empty list".to_string())
// }

// fn main() {
//     let mut my_nums = Vec::<i32>::new();
//     match fetch_last(&mut my_nums) {
//         Ok(ele) => println!("Last element: {ele}"),
//         Err(error) => {
//             println!("Error: {error}");
//             assert_eq!(error, "Empty list".to_owned());
//         }
//     }
// }

// Use `ok` combinator to convert Result to Option.
// Do not add any statements anywhere.

fn add(num1: &str, num2: &str) -> Option<u8> {
    // TODO: only modify the 2 statements below
    let num1 = num1.parse::<u8>().ok()?;
    let num2 = num2.parse::<u8>().ok()?;
    num1.checked_add(num2)
}

fn main() {
    let (num1, num2) = ("4", "5");
    if let Some(sum) = add("4", "5") {
        println!("{num1} + {num2} = {sum}");
    }
}
