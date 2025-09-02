use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m: HashMap<i32, i32> = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            match m.get(&(target - *v)) {
                Some(&i2) => return vec![i as i32, i2],
                None => m.insert(*v, i as i32),
            };
        }
        vec![]
    }
    pub fn two_sum_bubble(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let length = nums.len();

        for gap in 1..length {
            for right in gap..length {
                let left = right - gap;
                if nums[left] + nums[right] == target {
                    return vec![left as i32, right as i32];
                }
            }
        }

        vec![] // Return an empty vector if no solution found
    }
}
