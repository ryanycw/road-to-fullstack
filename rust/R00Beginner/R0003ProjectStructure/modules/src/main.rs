// // Something's missing. Fix the code so that it compiles.

// mod sausage_factory {
//     // Don't let anybody outside of this module see this!
//     fn get_secret_recipe() -> String {
//         String::from("Ginger")
//     }

//     pub fn make_sausage() {
//         get_secret_recipe();
//         println!("sausage!");
//     }
// }

// fn main() {
//     sausage_factory::make_sausage();
// }

// // Complete the following code by addressing the TODO.

// // TODO: Complete this use statement

// use std::time::{SystemTime, UNIX_EPOCH};

// fn main() {
//     match SystemTime::now().duration_since(UNIX_EPOCH) {
//         Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
//         Err(_) => panic!("SystemTime before UNIX EPOCH!"),
//     }
// }

// // Complete the code by bringing the required items into scope.

// use days::{WeekDay, is_holiday};

// mod days {
//     pub enum WeekDay {
//         Sunday,
//         Monday,
//         Tuesday,
//         Wednesday,
//         Thursday,
//         Friday,
//         Saturday,
//     }

//     pub fn is_holiday(day: &WeekDay) -> bool {
//         match day {
//             WeekDay::Sunday | WeekDay::Saturday => true,
//             _ => false,
//         }
//     }
// }

// fn main() {
//     let today = WeekDay::Friday;
//     if is_holiday(&today) {
//         println!("I can go out!");
//     } else {
//         println!("I have to work today!");
//     }
// }

// // Complete the code by use declarations above main.

// use student::{Student, operations::assign_grade};

// mod student {
//     pub mod operations {
//         use super::Student; // using super to refer to parent module

//         pub fn assign_grade(student: &mut Student) {
//             if student.marks >= 80 {
//                 student.grade = 'A';
//             } else if student.marks >= 60 {
//                 student.grade = 'B';
//             } else {
//                 student.grade = 'C';
//             }
//         }
//     }

//     pub struct Student {
//         pub name: String, // struct fields can also be made public
//         pub marks: u8,
//         pub grade: char,
//     }

//     impl Student {
//         // make methods/associated functions public in order to access from outside the module
//         pub fn new(name: &str, marks: u8) -> Self {
//             Self {
//                 name: name.to_string(),
//                 marks,
//                 grade: 'X',
//             }
//         }
//     }
// }

// fn main() {
//     let mut student = Student::new("Alice", 75);
//     assign_grade(&mut student);
//     println!("{} got {} grade", student.name, student.grade);
// }

// Make the code compile by addressing the TODO.

mod delicious_snacks {
    // TODO: Fix these use statements
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        // 'static just implies that reference will be valid throughout program execution
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
