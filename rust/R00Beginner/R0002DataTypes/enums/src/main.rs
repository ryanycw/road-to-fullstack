// // Complete the code by addressing the TODO.

// #[derive(Debug)] // this line makes the enum variants printable!
// enum Message {
//     // TODO: define a few types of messages as used below
//     Quit,
//     Echo,
//     Move,
//     ChangeColor,
// }

// fn main() {
//     println!("{:?}", Message::Quit);
//     println!("{:?}", Message::Echo);
//     println!("{:?}", Message::Move);
//     println!("{:?}", Message::ChangeColor);
// }

// Complete the code by addressing the TODO.

#[derive(Debug)] // this line makes the enum variants printable!
enum Message {
    // TODO: define the different variants used below
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
