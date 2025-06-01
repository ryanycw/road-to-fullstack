// // Fix the code so it compiles!

// const NUMBER: i16 = 3;

// fn main() {
//     println!("Number {}", NUMBER);
// }

// Fix the code so it compiles!

static LANGUAGE: &str = "Golang";

fn main() {
    // These two initializations perform a string copy from same memory location
    let lang1 = LANGUAGE;
    let mut lang2 = LANGUAGE;

    lang2 = "Rust";
    println!("I like {} more than {}!", lang2, lang1);
}

// Looking at this Rust code, the main issue is with the **misleading comment**. Let me explain what's actually happening:

// ```rust
// static LANGUAGE: &str = "Golang";

// fn main() {
//     // These two initializations perform a string copy from same memory location
//     let lang1 = LANGUAGE;
//     let mut lang2 = LANGUAGE;

//     lang2 = "Rust";
//     println!("I like {} more than {}!", lang2, lang1);
// }
// ```

// ## The Problem

// The comment says "These two initializations perform a string copy from same memory location", but this is **incorrect**. Here's what's actually happening:

// 1. **No string copying occurs** - `lang1` and `lang2` are both of type `&str` (string slices), which are just references/pointers to string data
// 2. Initially, both `lang1` and `lang2` point to the same static string `"Golang"`
// 3. When you assign `lang2 = "Rust"`, you're simply making `lang2` point to a different string literal - no copying happens
// 4. `lang1` continues to point to `"Golang"` while `lang2` now points to `"Rust"`

// ## What's Actually Happening

// - String literals like `"Golang"` and `"Rust"` are stored in the program's read-only memory
// - `&str` variables are just references (pointers) to these memory locations
// - Assignment changes what the reference points to, not the underlying string data

// The code will compile and run correctly, outputting: `I like Rust more than Golang!`

// ## Corrected Comment

// A more accurate comment would be:
// ```rust
// // These two variables are references pointing to the same static string
// let lang1 = LANGUAGE;
// let mut lang2 = LANGUAGE;
// ```

// The code functionality is fine, but the comment creates confusion about Rust's string handling and memory model.
