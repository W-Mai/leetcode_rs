/**
 * [000001] Hello
 *
 * World
 */
pub struct Solution {}



// problem:
// discuss: 

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let sum = nums[left] + nums[right];
            if sum == target {
                return vec![nums[left], nums[right]];
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
        vec![]
    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0000001() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![2, 7]);
    }
}