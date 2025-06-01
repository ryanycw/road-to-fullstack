// // Complete the code by addressing the TODO.

// struct User {
//     // TODO: Something goes here
//     name: String,
//     age: u8,
// }

// fn main() {
//     let user = User {
//         name: String::from("Tom Riddle"),
//         age: 17u8,
//     };
//     println!("User's name: {}", user.name);
//     println!("User's age: {}", user.age);
// }

// // Make the following code compile.

// struct ShopItem {
//     name: String,
//     quantity: u32,
//     in_stock: bool,
// }

// fn main() {
//     let mut item = ShopItem {
//         name: String::from("Socks"),
//         quantity: 200,
//         in_stock: true,
//     };
//     // 50 pairs of socks were sold
//     item.quantity -= 50;
//     if item.quantity == 0 {
//         item.in_stock = false;
//     }
//     println!("{} is in stock: {}", item.name, item.in_stock);
// }

// Complete the function signatures and make the code compile.

struct ShopItem {
    name: String,
    quantity: u32,
}

fn main() {
    let item = create_item("Socks", 200);
    let in_stock = is_in_stock(&item);
    println!("{} is in stock: {in_stock}", item.name);
}

fn create_item(name: &str, quantity: u32) -> ShopItem {
    ShopItem {
        name: name.to_string(),
        quantity, // notice how struct initializations can be shortened when variable and field have same name
    }
}

fn is_in_stock(item: &ShopItem) -> bool {
    item.quantity > 0
}
