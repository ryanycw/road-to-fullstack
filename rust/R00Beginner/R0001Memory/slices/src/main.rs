// // Complete the function definition to make the code compile.
// // The output of the code should be logically correct.

// fn main() {
//     let text = String::from("Today is a very warm and sunny day.");
//     let words = ["very", "arm", "say", "sun", "dew"];
//     let mut pos;

//     println!("Text: {text}");
//     for word in words {
//         pos = find_substr_pos(&text, word);
//         if pos == text.len() {
//             println!("{word} is not present in text");
//         } else {
//             println!("{word} present at index {pos}");
//         }
//     }
// }

// // this function tries to search for substr in text from left to right
// // if it finds substr, it returns the index where it starts
// // otherwise it returns length of text (which is an invalid index)
// fn find_substr_pos(text: &str, substr: &str) -> usize {
//     // both parameters should have same data type
//     if text.len() < substr.len() {
//         return text.len();
//     }
//     let len = substr.len();
//     for start in 0..text.len() - len + 1 {
//         if substr == &text[start..start + len] {
//             // what will be the correct range?
//             return start;
//         }
//     }
//     text.len()
// }

// Fix the program so that it compiles successfully and produces the desired output.

fn main() {
    let nums = [1, 1, 2, 3, 5, 8, 13];
    let res: (usize, usize) = find_subarray(&nums[..], 16);
    if res.0 == nums.len() {
        println!("No subarray found");
    } else {
        println!("Subarray found: {:?}", &nums[res.0..res.0 + res.1]);
    }
}

// this function searches an array to find a subarray with the given sum
// it returns the index where the subarray starts along with the length of the subarray
// if the array does not include any subarray with the sum, it returns a tuple with length or array
fn find_subarray(nums: &[i32], sum: i32) -> (usize, usize) {
    for len in (1..nums.len() + 1).rev() {
        for start in 0..nums.len() - len + 1 {
            if array_sum(&nums[start..start + len]) == sum {
                return (start, len);
            }
        }
    }
    (nums.len(), nums.len())
}
fn array_sum(nums: &[i32]) -> i32 {
    let mut res = 0;
    for num in nums {
        res += num;
    }
    res
}
