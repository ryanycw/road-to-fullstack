// // Fix the code so that it compiles.

// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let y: Option<Point> = Some(Point { x: 100, y: 200 });

//     match &y {
//         Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
//         _ => println!("no match"),
//     }
//     y; // Fix without deleting this line.
// }

// // Fix the code so that it compiles.

// fn last_element(nums: &[i32]) -> Option<i32> {
//     if nums.len() > 0 {
//         Some(nums[nums.len() - 1])
//     } else {
//         None
//     }
// }

// fn main() {
//     let my_nums = [1, 1, 2, 3, 5, 8, 13];
//     match last_element(&my_nums) {
//         Some(ele) => println!("Last element: {ele}"),
//         None => println!("Empty array"),
//     }
// }

// Fix the code so that it compiles.

struct User {
    id: u32,
    name: String,
}

fn get_user_name(id: u32) -> Option<String> {
    let database = [
        User {
            id: 1,
            name: String::from("Alice"),
        },
        User {
            id: 2,
            name: String::from("Bob"),
        },
        User {
            id: 3,
            name: String::from("Cindy"),
        },
    ];
    for user in database {
        if user.id == id {
            return Some(user.name);
        }
    }
    None
}

fn main() {
    let user_id = 3;
    if let Some(name) = get_user_name(user_id) {
        println!("User's name: {name}");
    }
}
