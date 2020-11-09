impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let first = nums[0];
        for i in nums.into_iter() {
            if i < first { return i; }
        }
        return first;
    }
}