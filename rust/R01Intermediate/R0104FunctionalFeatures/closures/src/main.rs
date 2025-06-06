// // Define the `double` closure & make the code execute successfully.

// fn main() {
//     let double = |input| input * 2;
//     assert_eq!(double(5), 10);
//     assert_eq!(double(-10), -20);
// }

// // Complete the struct definition & the implementation block.

// struct BinaryOperation<T, U>
// where
//     T: Copy,
//     U: Fn(T, T) -> T,
// {
//     operand1: T,
//     operand2: T,
//     operation: U,
// }

// impl<T, U> BinaryOperation<T, U>
// where
//     T: Copy,
//     U: Fn(T, T) -> T,
// {
//     fn calculate(&self) -> T {
//         (self.operation)(self.operand1, self.operand2)
//     }
// }

// fn main() {
//     let multiply = BinaryOperation {
//         operand1: 20,
//         operand2: 6,
//         operation: |a, b| a * b,
//     };
//     let divide = BinaryOperation {
//         operand1: 22.0,
//         operand2: 7.0,
//         operation: |x, y| x / y,
//     };
//     println!(
//         "{} x {} = {}",
//         multiply.operand1,
//         multiply.operand2,
//         multiply.calculate()
//     );
//     println!(
//         "{} / {} = {}",
//         divide.operand1,
//         divide.operand2,
//         divide.calculate()
//     );
// }

// // Something is wrong with the struct definition. Can you fix it?

// struct Manipulator<T>
// where
//     T: FnMut(),
// {
//     operation: T,
// }

// impl<T> Manipulator<T>
// where
//     T: FnMut(),
// {
//     fn manipulate(&mut self) {
//         (self.operation)()
//     }
// }
// fn main() {
//     let mut x = 0;
//     let mut y = 100;
//     let mut x_manipulator = Manipulator {
//         operation: || x += 1,
//     };
//     let mut y_manipulator = Manipulator {
//         operation: || y /= 5,
//     };
//     x_manipulator.manipulate();
//     x_manipulator.manipulate();
//     y_manipulator.manipulate();
//     y_manipulator.manipulate();
//     assert_eq!(x, 2);
//     assert_eq!(y, 4);
// }

// // Make the code compile by addressing the TODO.

// fn main() {
//     let my_str = "Hi there!".to_owned();
//     let substr = "here";
//     println!("String: {my_str}");
//     let check_substr = move |sub: &str| my_str.contains(sub);
//     let res = check_substr(substr);
//     // TODO: shift the statement below to somewhere else
//     println!("String contains {substr} : {res}");
// }

// // Complete the function signature.

// fn average<T, U>(nums: &[i32], add: T, div: U) -> f32
// where
//     T: Fn(i32, i32) -> i32,
//     U: Fn(i32, i32) -> f32,
// {
//     let mut sum = 0;
//     for num in nums {
//         sum = add(sum, *num);
//     }
//     div(sum, nums.len() as i32)
// }

// fn main() {
//     let add = |a, b| a + b;
//     let div = |a, b| a as f32 / b as f32;
//     let my_nums = [1, 2, 3, 4, 5];
//     let res = average(&my_nums, add, div);
//     println!("Average of {my_nums:?} = {res}");
// }

// // Fix the code to make it compile.

// fn main() {
//     let adder1 = get_adder(-2);
//     let adder2 = get_adder(100);
//     assert_eq!(adder1(20), 18);
//     assert_eq!(adder2(10), 110);
// }

// fn get_adder(to_add: i32) -> impl Fn(i32) -> i32 {
//     move |x| x + to_add
// }

// Fix the code to make it compile.

enum Operation {
    Add,
    Sub,
    Mul,
}

fn get_implementation(operation: Operation, operand2: i32) -> Box<dyn Fn(i32) -> i32> {
    match operation {
        Operation::Add => Box::new(move |x| x + operand2),
        Operation::Sub => Box::new(move |x| x - operand2),
        Operation::Mul => Box::new(move |x| x * operand2),
    }
}

fn main() {
    const OPERAND2: i32 = 5;
    let adder = get_implementation(Operation::Add, OPERAND2);
    let multiplier = get_implementation(Operation::Mul, OPERAND2);
    assert_eq!(adder(10), 15);
    assert_eq!(multiplier(10), 50);
}
