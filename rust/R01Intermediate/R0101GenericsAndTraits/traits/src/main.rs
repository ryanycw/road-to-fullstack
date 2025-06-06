// // Complete the code by addressing the TODO.

// trait AppendBar {
//     fn append_bar(self) -> Self;
// }

// impl AppendBar for String {
//     // TODO: Implement `AppendBar` for type `String`.
//     fn append_bar(self) -> Self {
//         format!("{self}Bar")
//     }
// }

// fn main() {
//     let s = String::from("Foo");
//     let s = s.append_bar();
//     println!("s: {}", s);
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn is_foo_bar() {
//         assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
//     }

//     #[test]
//     fn is_bar_bar() {
//         assert_eq!(
//             String::from("").append_bar().append_bar(),
//             String::from("BarBar")
//         );
//     }
// }

// // Complete the code by addressing the TODO.

// trait AppendBar {
//     fn append_bar(self) -> Self;
// }

// // TODO: Implement trait `AppendBar` for a vector of strings.
// impl AppendBar for Vec<String> {
//     fn append_bar(mut self) -> Self {
//         self.push("Bar".into());
//         self
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn is_vec_pop_eq_bar() {
//         let mut foo = vec![String::from("Foo")].append_bar();
//         assert_eq!(foo.pop().unwrap(), String::from("Bar"));
//         assert_eq!(foo.pop().unwrap(), String::from("Foo"));
//     }
// }

// // Fix this code by updating the Licensed trait.

// pub trait Licensed {
//     fn licensing_info(&self) -> String {
//         "Some information".to_owned()
//     }
// }

// struct SomeSoftware {
//     version_number: i32,
// }

// struct OtherSoftware {
//     version_number: String,
// }

// impl Licensed for SomeSoftware {} // Don't edit this line
// impl Licensed for OtherSoftware {} // Don't edit this line

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn is_licensing_info_the_same() {
//         let licensing_info = String::from("Some information");
//         let some_software = SomeSoftware { version_number: 1 };
//         let other_software = OtherSoftware {
//             version_number: "v2.0.0".to_string(),
//         };
//         assert_eq!(some_software.licensing_info(), licensing_info);
//         assert_eq!(other_software.licensing_info(), licensing_info);
//     }
// }

// fn main() {}

// Make the code execute successfully by correctly implementing Message trait for Cat type.

trait Message {
    fn message(&self) -> String {
        "Default Message!".to_string()
    }
}

struct Fish;
struct Cat;
impl Message for Fish {}
impl Message for Cat {
    fn message(&self) -> String {
        format!("Meow 🐱")
    }
}

fn main() {
    let fish = Fish;
    let cat = Cat;
    assert_eq!(String::from("Default Message!"), fish.message());
    assert_eq!(String::from("Meow 🐱"), cat.message());
}
