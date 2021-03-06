use std::cmp::{min, max};

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        
        // O(n).
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut max_v = 0;
        
        while l != r {
            max_v = max(max_v, min(height[l], height[r])*((r-l) as i32));
            if height[r] < height[l] { r -= 1; }
            else { l += 1; }
        }
        
        max_v
    }
}