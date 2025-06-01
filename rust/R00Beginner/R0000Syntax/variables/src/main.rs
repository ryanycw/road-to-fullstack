// Fix the variable definition of 'x'

fn main() {
    // Exercise 1
    // let x = 5;
    // println!("x has the value {}", x);

    // Exercise 2
    // let mut x = 3;
    // println!("Number {}", x);
    // x = 5; // don't change this line
    // println!("Number {}", x);

    // Exercise 3
    // Fix this code with shadowing
    let x = "three"; // don't change this line
    println!("Spell a Number : {}", x);
    let x = 3; // don't rename this variable
    println!("Number plus two is : {}", x + 2);
}
