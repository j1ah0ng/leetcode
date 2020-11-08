use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            if map.get(&(target - n)).is_some() {
                return vec![i as i32, *map.get(&(target - n)).unwrap_or(&0) as i32]
            } else {
                map.insert(*n, i);
            }
        }
        vec![0, 0]
    }
}