// // Make the following code compile.
// // If you score 50 or less, you fail.

// fn main() {
//     // marks scored out of 100
//     let marks = 75u8;
//     match marks {
//         91..=100 => println!("You performed excellent!"),
//         71..=90 => println!("You performed good :)"),
//         51..=70 => println!("Your performance was average..."),
//         0..=50 => println!("You failed. Better luck next time."),
//         101..=u8::MAX => println!("Invalid marks!!!"),
//     }
// }

// // Fix the code so that it compiles.

// fn main() {
//     let side_count = 5;
//     let message = match side_count {
//         0 | 1 | 2 => "invalid shape",
//         3 => "it's a triangle",
//         4 => "it's a quadrilateral",
//         5 => "it's a pentagon",
//         6 => "it's a hexagon",
//         _ => "i don't know the name, lol",
//     };
//     println!("{message}");
// }

// // Fix the code so that it compiles.

// // USD coin types
// // cent values: penny:1, nickel:5, dime: 10, quarter:25
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn main() {
//     let piggy_bank = [Coin::Nickel, Coin::Penny, Coin::Dime, Coin::Penny];
//     let mut my_savings = 0;
//     for coin in piggy_bank {
//         my_savings += value_in_cents(coin);
//     }
//     println!("My savings: {my_savings} cents");
// }

// Fix the code so that it compiles.

enum Operation {
    Add(u8, u8),
    Mul(u8, u8),
    Subtract { first: u8, second: u8 },
    Divide { divident: u8, divisor: u8 },
}

impl Operation {
    fn result(&self) -> u8 {
        match self {
            Self::Add(a, b) => a + b, // notice Self can be used instead of Operation
            Self::Mul(a, b) => a * b,
            Self::Subtract { first, second } => first - second,
            Self::Divide { divident, divisor } => divident / divisor,
        }
    }
}

fn main() {
    let user_operation = Operation::Subtract {
        first: 75,
        second: 20,
    };
    println!("Result: {}", user_operation.result());
}
