#[cfg(test)]
mod tests_increments {
    use super::*;

    #[test]
    fn incrementing() {
        let items = vec![1, 2, 3, 4];
        assert_eq!(count_increments(&items), 3);
    }

    #[test]
    fn aoc_example() {
        let items = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increments(&items), 7);
    }

    #[test]
    fn decrementing() {
        let items = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        assert_eq!(count_increments(&items), 0);
    }

    #[test]
    fn negative_mixed() {
        let items = vec![0, -1, -2, -3, -4, -3, 18];
        assert_eq!(count_increments(&items), 2);
    }

    #[test]
    fn windows_aoc_example() {
        let items = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increments_windows(&items), 5);
    }
}

pub fn count_increments(nums: &Vec<i32>) -> i32 {
    nums.iter()
        .zip(nums.iter().skip(1))
        .filter(|(current, next)| current < next)
        .count() as i32
}

pub fn count_increments_windows(nums: &Vec<i32>) -> i32 {
    let mut accum = Vec::new();

    for i in 0..nums.len() - 2 {
        let num = nums[i];

        accum.push(num + nums[i + 1] + nums[i + 2])
    }

    count_increments(&accum)
}
