impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        
        // We want to numerically find the local minimum and local maximum s.t. the
        // local minimum occurs before the local maximum. 
        let mut min_index: usize = 0;
        let mut max_pft: i32 = 0;
        for (i, p) in prices.iter().enumerate() {
            
            // Decreasing. Find maximum jump from here
            if p < &prices[min_index] { 
                min_index = i;
                continue;
            } 
            // Increasing. Find maximum jump from previous minimum.
            else {
                if max_pft < prices[i] - prices[min_index] {
                    max_pft = prices[i] - prices[min_index];
                }
            }
        }
        
        max_pft
    }
}