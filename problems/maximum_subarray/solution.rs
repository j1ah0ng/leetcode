use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_ending: i32 = nums[0];
        let mut abs_max: i32 = nums[0];
        for i in 1..nums.len() {
            // max_ending ie the maximum array ending at the current
            // index (exclusive) is either a continuation of the current run, 
            // or has already decreased, in which case zero it 
            max_ending = max(nums[i], max_ending + nums[i]);
            abs_max = max(max_ending, abs_max)
        }
        abs_max
    }
}