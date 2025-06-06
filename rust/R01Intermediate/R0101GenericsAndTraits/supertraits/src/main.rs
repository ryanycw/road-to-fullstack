// // Make the code compile. Large should be the default size.

// /*
//    Default trait is provided by standard library.
//    Has one associated function: default() -> Self
// */
// trait Bounded {
//     fn get_max() -> Self;
//     fn get_min() -> Self;
// }

// #[derive(Default)]
// enum Size {
//     Small,
//     Medium,
//     #[default]
//     Large,
// }

// impl Bounded for Size {
//     fn get_max() -> Self {
//         Self::Large
//     }
//     fn get_min() -> Self {
//         Self::Small
//     }
// }

// fn get_size_num(size: &Size) -> u8 {
//     match size {
//         Size::Small => 0,
//         Size::Medium => 1,
//         Size::Large => 2,
//     }
// }

// fn main() {
//     let my_size = Size::Large;
//     let min_size_num = get_size_num(&Size::get_min());
//     let default_size_num = get_size_num(&Size::default());
//     let my_size_num = get_size_num(&my_size);
//     if my_size_num == min_size_num {
//         println!("I have the shortest size!");
//     }
//     if my_size_num == default_size_num {
//         println!("Default size suits me!")
//     }
// }

// Something is missing with the definition of Comparable trait. Fix it.

trait Numeric {
    fn convert_to_num(&self) -> u8;
}

trait Printable {
    fn convert_to_str(&self) -> String;
}

trait Comparable: Numeric + Printable {
    fn print_greater(a: &Self, b: &Self) {
        let num1 = a.convert_to_num();
        let num2 = b.convert_to_num();
        if num1 > num2 {
            println!(
                "{} is greater than {}",
                a.convert_to_str(),
                b.convert_to_str()
            );
        } else if num2 > num1 {
            println!(
                "{} is greater than {}",
                b.convert_to_str(),
                a.convert_to_str()
            );
        } else {
            println!("Both sizes are {}", a.convert_to_str());
        }
    }
}

enum Size {
    Small,
    Medium,
    Large,
}

impl Numeric for Size {
    fn convert_to_num(&self) -> u8 {
        match self {
            Self::Small => 0,
            Self::Medium => 1,
            Self::Large => 2,
        }
    }
}

impl Printable for Size {
    fn convert_to_str(&self) -> String {
        match self {
            Self::Small => "Small size".to_string(),
            Self::Medium => "Medium size".to_string(),
            Self::Large => "Large size".to_string(),
        }
    }
}

impl Comparable for Size {}

fn main() {
    let (size1, size2) = (Size::Small, Size::Medium);
    Comparable::print_greater(&size1, &size2);
}
