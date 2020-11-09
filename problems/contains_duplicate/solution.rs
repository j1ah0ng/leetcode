use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {

        let mut map: HashSet<i32> = HashSet::<i32>::new();
        for i in nums.into_iter() {
            if !map.insert(i) { return true }
        }
        
        false
    }
}