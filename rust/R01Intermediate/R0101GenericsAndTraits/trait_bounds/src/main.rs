// // Complete the function signatures.

// trait Message {
//     fn message(&self) -> String {
//         "I love Rust 🦀".to_string()
//     }
// }

// fn print_msg1<T: Message>(input: &T) {
//     println!("{}", input.message());
// }

// fn print_msg2(input: &impl Message) {
//     println!("{}", input.message());
// }

// fn print_msg3<T>(input: &T)
// where
//     T: Message,
// {
//     println!("{}", input.message());
// }

// struct Dummy;

// impl Message for Dummy {}

// fn main() {
//     let var = Dummy;
//     print_msg1(&var);
//     print_msg2(&var);
//     print_msg3(&var);
// }

// // Make the code compile by completing the function signature of `print_message`.

// trait Message {
//     fn message(&self) -> String {
//         "How are you?".to_string()
//     }
// }

// trait Printer {
//     fn print(&self, printable: &impl Message) {
//         println!("Message is: {}", printable.message());
//     }
// }

// struct M;
// struct P;

// impl Message for M {}
// impl Printer for P {}

// fn print_message<T, U>(msg: &T, printer: &U)
// where
//     T: Message,
//     U: Printer,
// {
//     printer.print(msg);
// }

// fn main() {
//     let m = M;
//     let p = P;
//     print_message(&m, &p);
// }

// // Complete the code so that it compiles.

// pub trait Licensed {
//     fn licensing_info(&self) -> String {
//         "some information".to_string()
//     }
// }

// struct SomeSoftware {}

// struct OtherSoftware {}

// impl Licensed for SomeSoftware {}
// impl Licensed for OtherSoftware {}

// // YOU MAY ONLY CHANGE THE NEXT LINE
// fn compare_license_types(software: impl Licensed, software_two: impl Licensed) -> bool {
//     software.licensing_info() == software_two.licensing_info()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn compare_license_information() {
//         let some_software = SomeSoftware {};
//         let other_software = OtherSoftware {};

//         assert!(compare_license_types(some_software, other_software));
//     }

//     #[test]
//     fn compare_license_information_backwards() {
//         let some_software = SomeSoftware {};
//         let other_software = OtherSoftware {};

//         assert!(compare_license_types(other_software, some_software));
//     }
// }

// // Make the code compile by completing the function signature of `get_double_str`.

// trait Double {
//     fn double(&self) -> Self;
// }

// trait Printable {
//     fn convert_to_str(self) -> String;
// }

// fn get_double_str(input: impl Double + Printable) -> String {
//     let doubled = input.double();
//     doubled.convert_to_str()
// }

// impl Double for i32 {
//     fn double(&self) -> Self {
//         2 * self
//     }
// }

// impl Printable for i32 {
//     fn convert_to_str(self) -> String {
//         format!("{self}")
//     }
// }

// fn main() {
//     let num = 22;
//     let mut msg = format!("{num} doubled is ");
//     msg.push_str(&get_double_str(num));
//     println!("{msg}");
// }

// Complete the code so that it compiles.

// pub trait SomeTrait {
//     fn some_function(&self) -> bool {
//         true
//     }
// }

// pub trait OtherTrait {
//     fn other_function(&self) -> bool {
//         true
//     }
// }

// struct SomeStruct {}
// struct OtherStruct {}

// impl SomeTrait for SomeStruct {}
// impl OtherTrait for SomeStruct {}
// impl SomeTrait for OtherStruct {}
// impl OtherTrait for OtherStruct {}

// // YOU MAY ONLY CHANGE THE NEXT LINE
// fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
//     item.some_function() && item.other_function()
// }

// fn main() {
//     some_func(SomeStruct {});
//     some_func(OtherStruct {});
// }

// Complete function signature of `get_bigger`.
// Only Addable trait's functions should be callable on its return value.

trait Addable {
    fn add_one(&self) -> Self;
    fn are_equal(a: &Self, b: &Self) -> bool;
}

impl Addable for u8 {
    fn add_one(&self) -> Self {
        if *self == u8::MAX { *self } else { self + 1 }
    }
    fn are_equal(a: &Self, b: &Self) -> bool {
        a == b
    }
}

fn get_bigger(a: u8, b: u8) -> impl Addable {
    if a > b { a } else { b }
}

fn main() {
    let (num1, num2) = (125, 220);
    let bigger = get_bigger(num1, num2);
    if Addable::are_equal(&bigger, &bigger.add_one()) {
        println!("Bigger number has max value")
    } else {
        println!("Both numbers are smaller than max value");
    }
}
