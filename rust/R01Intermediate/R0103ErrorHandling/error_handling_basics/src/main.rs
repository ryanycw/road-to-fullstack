// // Modify `get_last` to handle all cases and make the program execute successfully.

// fn get_last(nums: &mut Vec<i32>) -> Result<i32, &'static str> {
//     if nums.is_empty() {
//         return Err("Empty vector");
//     }
//     Ok(nums.pop().unwrap())
// }

// fn main() {
//     let mut vec1 = vec![1, 2, 3];
//     let mut vec2 = vec![];
//     assert!(matches!(get_last(&mut vec1), Ok(3)));
//     assert!(matches!(get_last(&mut vec2), Err("Empty vector")));
// }

// Modify the functions to propagate the error instead of panicking.

fn factorial(n: u32) -> Result<u32, String> {
    if n == 0 {
        return Ok(1);
    } else if n > 12 {
        // Factorial of values > 12 would overflow u32, so return an error
        return Err(String::from("Input too large"));
    }
    let result = n * factorial(n - 1).unwrap();
    Ok(result)
}

fn print_factorial(n: u32) -> Result<(), String> {
    let result = factorial(n)?;
    println!("Factorial of {} is: {}", n, result);
    Ok(())
}

fn main() {
    let n = 13;
    if let Err(err) = print_factorial(n) {
        eprintln!("Error calculating factorial of {}: {}", n, err);
    }
}
