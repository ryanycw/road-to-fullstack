pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn calculate_sum(nums: &[i32]) -> Result<i32, String> {
    if nums.len() == 0 {
        return Err("Number list is empty".to_string());
    }
    let mut sum = 0;
    for num in nums {
        sum += num;
    }
    Ok(sum)
}

fn average(nums: &[i32]) -> i32 {
    if nums.len() == 0 {
        panic!("Empty number list");
    }
    let mut sum = 0;
    for num in nums {
        sum += num;
    }
    sum / nums.len() as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(0, 0);
    }

    #[test]
    fn you_can_assert() {
        assert!(true);
    }

    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(1));
    }

    #[test]
    fn calculates_sum_correctly() -> Result<(), String> {
        let nums = [1, 2, 3, 4, 5];
        let sum = calculate_sum(&nums)?;
        assert_eq!(sum, 5 * (5 + 1) / 2);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn it_panics() {
        let nums = [];
        let _avg = average(&nums);
    }
}

fn main() {}
