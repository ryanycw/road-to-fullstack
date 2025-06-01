// // Fix the code so that it compiles.

// fn main() {
//     let mut str1 = String::from("modifiable");
//     let str2 = String::from("fixed string");
//     let mut str_ptr: &String;
//     str_ptr = &str1;
//     println!("ptr currently points to {str_ptr}");
//     str_ptr = &str2;
//     println!("ptr currently points to {str_ptr}");
//     str1.push_str(" string");
//     str_ptr = &str1;
//     println!("ptr currently points to {str_ptr}");
// }

// // Fix the code so that it compiles.

// fn main() {
//     let mut s = String::from("Hello, ");
//     let s_ref = &mut s;
//     change_string(s_ref);
//     println!("{s_ref}");
// }

// fn change_string(s: &mut String) {
//     s.push_str(" world!");
// }

// // Fix the code so that it compiles.

// fn main() {
//     let mut str1 = String::from("Rust");
//     let mut str2 = String::from("Golang");
//     let ref1 = &mut str1;
//     let mut ref2 = &mut str2;

//     println!("First string: {ref1}");
//     println!("Second string: {ref2}");
//     ref1.push('🦀');
//     ref2.push('🦫');
//     println!("Modified first string: {ref1}");
//     println!("Modified second string: {ref2}");
//     // only one mutable reference allowed at a time, ref1 is no longer valid
//     ref2 = &mut str1;
//     ref2.pop();
//     println!("Original first string: {ref2}");
// }

// Complete the function signature to make the code compile.

fn main() {
    let mut s1 = String::from("this is ");
    let s2 = String::from("an example sentence");
    concat(&mut s1, &s2);
    println!("{s1}")
}

fn concat(s1: &mut String, s2: &str) {
    for ch in s2.chars() {
        s1.push(ch);
    }
}
