use std::cmp::max;

impl Solution {
    pub fn longest_mountain(a: Vec<i32>) -> i32 {
        
        if a.len() < 3 { return 0; }
        
        let len = a.len();
        
        // Track the start and the end of a mountain.
        let mut begin: usize = 0;
        let mut end: usize = 0;
        let mut max_peak: usize = 0;
        
        while begin < len {
            end = begin;
            if end + 1 < len && a[end] < a[end + 1] {
                // Find peak
                while end + 1 < len && a[end] < a[end + 1] { end += 1; }
                // Ensure this is not the end
                if end + 1 < len && a[end] > a[end + 1] {
                    while end + 1 < len && a[end] > a[end + 1] { end += 1; }
                    max_peak = max(max_peak, end - begin + 1);
                }
            }
            if begin == end {
                begin += 1;
            } else {
                begin = end;
            }
        }
        
        max_peak as i32
    }
}