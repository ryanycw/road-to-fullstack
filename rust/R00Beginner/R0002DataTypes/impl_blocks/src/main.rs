// // Complete the method signatures by providing appropriate arguments.

// struct Student {
//     first_name: String,
//     last_name: String,
//     roll_no: u16,
// }

// impl Student {
//     fn get_name(&self) -> String {
//         format!("{} {}", self.first_name, self.last_name)
//     }
//     fn set_roll_no(&mut self, new_roll_no: u16) {
//         self.roll_no = new_roll_no;
//     }
//     fn convert_to_string(self) -> String {
//         // should take ownership
//         format!(
//             "Name: {} {}, Roll no: {}",
//             self.first_name, self.last_name, self.roll_no
//         )
//     }
// }

// fn main() {
//     let mut student = Student {
//         first_name: "Harry".to_string(),
//         last_name: "Potter".to_string(),
//         roll_no: 42,
//     };
//     println!("Student is: {}", student.get_name());
//     student.set_roll_no(50);
//     let student_details = student.convert_to_string();
//     println!("{student_details}");
// }

// Fix the code so that it compiles.

struct ShopItem {
    name: String,
    quantity: u32,
}

impl ShopItem {
    fn new(name: String, quantity: u32) -> ShopItem {
        ShopItem { name, quantity }
    }
    fn in_stock(&self) -> bool {
        self.quantity > 0
    }
}

fn main() {
    let item = ShopItem::new("Pants".to_string(), 450);
    if item.in_stock() {
        println!("{} remaining: {}", item.name, item.quantity);
    } else {
        println!("{} not in stock", item.name);
    }
}
