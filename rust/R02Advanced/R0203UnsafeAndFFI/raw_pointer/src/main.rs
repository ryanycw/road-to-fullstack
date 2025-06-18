// // Fix the code to make it compile. You may not modify any statement.

// fn main() {
//     let num = 123;
//     let ptr = &num as *const i32;
//     unsafe {
//         println!("{} stored at {:p}", *ptr, ptr);
//     }
// }

// // Fix the code to make it compile.

// fn main() {
//     // print first 10 fibonacci numbers
//     let (mut a, mut b) = (1, 0);
//     let mut c = 0;
//     let ptr_a = &mut a as *mut i32;
//     let ptr_b = &mut b as *mut i32;
//     let ptr_c = &mut c as *mut i32;
//     unsafe {
//         for _ in 0..10 {
//             *ptr_c = *ptr_a + *ptr_b;
//             println!("{}", *ptr_c);
//             *ptr_a = *ptr_b;
//             *ptr_b = *ptr_c;
//         }
//     }
// }

// Fix the code to make it compile.

macro_rules! ptr {
    ($type:ty, $var:ident) => {
        &mut $var as *mut $type
    };
}

fn main() {
    let mut x = 20;
    let ptr1 = ptr!(i32, x);
    let ptr2 = ptr!(i32, x);
    println!("x: {x}");
    unsafe {
        *ptr1 = *ptr1 * 2;
        *ptr2 = *ptr2 * 2;
        *ptr2 = *ptr1 * 2;
    }
    println!("x * 8 = {x}");
}
