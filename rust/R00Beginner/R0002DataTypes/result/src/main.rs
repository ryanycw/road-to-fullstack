// // Complete the match statement to check whether operation succeeded or not.

// enum Operation {
//     Add(i32, i32),
//     Mul(i32, i32),
//     Sub { first: i32, second: i32 },
//     Div { divident: i32, divisor: i32 },
// }

// impl Operation {
//     fn execute(self) -> Result<i32, String> {
//         match self {
//             Self::Add(a, b) => Ok(a + b),
//             Self::Mul(a, b) => Ok(a * b),
//             Self::Sub { first, second } => Ok(first - second),
//             Self::Div { divident, divisor } => {
//                 if divisor == 0 {
//                     Err(String::from("Can not divide by zero"))
//                 } else {
//                     Ok(divident / divisor)
//                 }
//             }
//         }
//     }
// }

// fn main() {
//     let user_input = Operation::Div {
//         divident: 20,
//         divisor: 0,
//     };
//     match user_input.execute() {
//         Ok(res) => println!("Result: {res}"),
//         Err(e) => println!("Error: {e}"),
//     }
// }

// Complete the function signature.

fn greet(name: &str) -> Result<(), String> {
    if name.len() > 0 {
        println!("Hello {name}!");
        Ok(())
    } else {
        Err("Empty name provided".to_string())
    }
}

fn main() {
    let name = "Tom";
    if let Err(e) = greet(name) {
        println!("Error: {e}");
    }
}
