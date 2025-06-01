fn main() {
    // Exercise 1
    // Something's missing. Fix the code so that it compiles.
    // let is_morning = true;
    // if is_morning {
    //     println!("Good morning!");
    // }

    // let is_evening = true; // Finish the rest of this line
    // if is_evening {
    //     println!("Good evening!");
    // }

    // Exercise 2
    // Do you really have that few friends?
    // Assign the correct value to the variable.
    // let number_of_friends: u32; // Don't change this line!
    // number_of_friends = 1;
    // println!("I have {} friends!", number_of_friends);

    // Exercise 3
    // Make this program compile by replacing the variable type.
    // let number_of_stars: i64;
    // number_of_stars = 400_000_000_000; // The Milky Way has more stars than can fit in a 32-bit integer type!
    // println!(
    //     "There are about {} stars in the Milky Way galaxy!",
    //     number_of_stars
    // );

    // Exercise 4
    // Assign the correct data types to the variables
    // let pi2: f32;
    // pi2 = 3.14;
    // println!("Pi is {}, correct to 2 decimal places.", pi2);

    // // What if we want to be more precise with our floating-point numbers?

    // let pi15: f64 /* Give this variable a data type! */;
    // pi15 = 3.141592653589793;
    // println!("Pi is {}, correct to 15 decimal places.", pi15);

    // Exercise 5
    // Fill in the rest of the line that has code missing!
    // let my_first_initial = 'C';

    // if my_first_initial.is_alphabetic() {
    //     println!("Alphabetical!");
    // } else if my_first_initial.is_numeric() {
    //     println!("Numerical!");
    // } else {
    //     println!("Neither alphabetic nor numeric!");
    // }

    // let your_character = '🙏'; // Finish this line like the example! What's your favorite character?
    // // Try a letter, try a number, try a special character, try a character
    // // from a different language than your own, try an emoji!

    // if your_character.is_alphabetic() {
    //     println!("Alphabetical!");
    // } else if your_character.is_numeric() {
    //     println!("Numerical!");
    // } else {
    //     println!("Neither alphabetic nor numeric!");
    //}

    // Exercise 6
    // Make this program compile without changing the variable type!
    // let answer: String = "blu".to_string(); /* Your favorite color here */
    // println!("My current favorite color is {}", answer);

    // Exercise 7
    // Create an array with at least 10 elements in it
    // let a = [10; 42]; /* Your array here */
    // if a.len() >= 10 {
    //     println!("Wow, that's a big array!");
    // }

    // Exercise 8
    // Destructure the `cat` tuple so that the println will work.
    // let cat = ("Furry McFurson", 3.5);
    // let (name, age)/* Your pattern here */ = cat;

    // println!("{} is {} years old.", name, age);

    // Exercise 9
    // Add a type alias for our dogs so we can store their names and ages
    type Dog = (String, i16); /* Finish this line */

    let dog1: Dog = (String::from("Albert"), 3);
    println!("My dog {} is {} years old.", dog1.0, dog1.1);

    let dog2: Dog = (String::from("Barker"), 5);
    println!("My other dog {} is {} years old.", dog2.0, dog2.1);
}
