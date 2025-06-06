// // Complete the function signature for `factorial`. It must not contain any generics/traits.

// fn decrement(x: u32) -> u32 {
//     x - 1
// }

// fn multiply(x: u32, y: u32) -> u32 {
//     x * y
// }

// fn factorial<T, U>(num: u32, dec: T, mul: U) -> u32
// where
//     T: Fn(u32) -> u32,
//     U: Fn(u32, u32) -> u32,
// {
//     let mut res = 1;
//     let mut temp = num;
//     while temp > 1 {
//         res = mul(res, temp);
//         temp = dec(temp);
//     }
//     res
// }

// fn main() {
//     let num = 6;
//     let fact = factorial(num, decrement, multiply);
//     println!("{num}! = {fact}");
// }

// Fix the code by addressing the TODO.

fn invoker<T>(logic: fn(T), arg: T) {
    logic(arg);
}

fn main() {
    // TODO: shift below declaration to somewhere else
    let greet = |name| {
        let greeting = String::from("Nice to meet you");
        println!("{greeting} {name}!");
    };
    invoker(greet, "Jenny");
}
