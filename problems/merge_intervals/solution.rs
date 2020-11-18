use std::cmp::{max, min};

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        
        let mut sorted = intervals.clone();
        sorted.sort_by(|a, b| b[0].cmp(&a[0]));
        
        let first = sorted.pop().expect("");
        let mut last_a = first[0];
        let mut last_b = first[1];
        let mut merged: Vec<Vec<i32>> = Vec::new();
        
        loop {
            if sorted.len() < 1 { break; }
            
            let next = sorted.pop().expect("");
            let a = next[0];
            let b = next[1];
            
            // Do we update the intervals?
            if a <= last_b {
                last_b = max(last_b, b);
            } else {
                // Disjoint.
                merged.push(vec!(last_a, last_b));
                last_a = a;
                last_b = b;
            }
        }
        
        // Do we push the last?
        if merged.len() == 0 || merged[merged.len() - 1][0] != last_a {
            merged.push(vec!(last_a, last_b));
        }
        
        merged
    }
}