impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        *(1..=n).filter(|x| n % x == 0)
            .collect::<Vec<i32>>()
            .get((k - 1) as usize)
            .unwrap_or(&-1)
    }
}