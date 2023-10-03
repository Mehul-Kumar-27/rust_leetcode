pub struct Solution;

impl Solution {
    pub fn is_sorted(nums: &Vec<i32>, rotated_index: i32) -> bool {
        let n: i32 = nums.len() as i32;
        let mut original_vector: Vec<i32> = vec![0; nums.len()];

        for i in 0..n {
            let original_index: i32 = (n + i - rotated_index) % n;
            original_vector[original_index as usize] = nums[i as usize];
        }

        let mut sorted_vector: Vec<i32> = original_vector.clone();
        sorted_vector.sort();

        if sorted_vector == original_vector {
            return true;
        } else {
            return false;
        }
    }
    pub fn check_rotated(nums: Vec<i32>) -> bool {
        let size = nums.len();
        let last_index = size as i32;
        let mut minimum = nums[0];
        let mut min_index: i32 = 0;

        for (i, &num) in nums.iter().enumerate() {
            if num < minimum {
                minimum = num;
                min_index = i as i32;
            }
        }
        if (min_index == 0) {
            if (nums[0] == nums[size - 1]) {
                if Solution::is_sorted(&nums, min_index) || Solution::is_sorted(&nums, last_index) {
                    return true;
                } else {
                    return false;
                }
            } else {
                if Solution::is_sorted(&nums, min_index) {
                    return true;
                } else {
                    return false;
                }
            }
        } else {
            if Solution::is_sorted(&nums, min_index) {
                return true;
            } else {
                return false;
            }
        }
    }
}
